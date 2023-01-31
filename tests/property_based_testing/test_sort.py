from __future__ import annotations

import os

import pandas as pd
from hypothesis import note, settings
from hypothesis.stateful import Bundle, RuleBasedStateMachine, precondition, rule
from hypothesis.strategies import (
    booleans,
    data,
    integers,
    lists,
    permutations,
    sampled_from,
)

from daft import DataFrame
from tests.conftest import assert_df_equals
from tests.property_based_testing.strategies import (
    columns_dict,
    row_nums_column,
    total_order_dtypes,
)


@settings(max_examples=int(os.getenv("HYPOTHESIS_MAX_EXAMPLES", 100)), stateful_step_count=16)
class DataframeSortStateMachine(RuleBasedStateMachine):
    """Tests sorts in the face of various other operations such as filters, projections etc

    Creates N number of sort key columns named "sort_key_{i}", and one "row_num" column which enumerates the original row number.

    Intermediate steps can consist of repartitions, filters, projections, sorts etc.

    The sort steps will additionally pull data down to check sort correctness.
    """

    Dataframes = Bundle("dataframes")

    def __init__(self):
        super().__init__()
        self.df: DataFrame | None = None
        self.sort_keys: list[str] = None
        self.row_num_col_name = "row_num"
        self.num_rows_strategy = integers(min_value=8, max_value=8)
        self.repartition_num_partitions_strategy = sampled_from([1, 4, 5, 9])
        self.sorted_on: list[tuple[str, bool]] | None = None

    @rule(data=data(), num_sort_cols=integers(min_value=1, max_value=3))
    @precondition(lambda self: self.df is None)
    def newdataframe(self, data, num_sort_cols):
        """Start node of the state machine, creates an initial dataframe"""
        self.sort_keys = [f"sort_key_{i}" for i in range(num_sort_cols)]

        # Generate N number of sort key columns, and one "row_num" column which enumerates the original row number
        columns_dict_data = data.draw(
            columns_dict(
                generate_columns_with_type={
                    sort_key_col_name: total_order_dtypes for sort_key_col_name in self.sort_keys
                },
                generate_columns_with_strategy={self.row_num_col_name: row_nums_column},
                num_rows_strategy=self.num_rows_strategy,
            )
        )
        df = DataFrame.from_pydict(columns_dict_data)
        self.df = df

    @rule(data=data())
    @precondition(lambda self: self.df is not None)
    def run_and_check_sort(self, data):
        """Run a sort on the accumulated dataframe plan"""
        sort_on = data.draw(permutations(self.sort_keys))
        descending = data.draw(lists(min_size=len(self.sort_keys), max_size=len(self.sort_keys), elements=booleans()))
        self.df = self.df.sort(sort_on, desc=descending)
        self.sorted_on = list(zip(sort_on, descending))

    @rule()
    @precondition(lambda self: self.sorted_on is not None)
    def collect_after_sort(self):
        """Optionally runs after any sort step to check that sort is maintained"""
        sorted_data = self.df.to_pydict()
        sorted_on_cols = [c for c, _ in self.sorted_on]
        sorted_on_desc = [d for _, d in self.sorted_on]

        # Ensure that key column(s) are sorted correctly
        pd_df_sorted = pd.DataFrame(sorted_data)
        sorted_keys_df = pd_df_sorted[sorted_on_cols]
        pandas_sorted_keys_df = sorted_keys_df.sort_values(sorted_on_cols, ascending=[not d for d in sorted_on_desc])
        try:
            assert_df_equals(sorted_keys_df, pandas_sorted_keys_df, assert_ordering=True)
        except AssertionError:
            note(f"Expected sorted keys:\n{pandas_sorted_keys_df}")
            note(f"Received sorted keys:\n{sorted_keys_df}")
            raise

        # Ensure that we reset self.sorted_on so that we won't try to collect again
        self.sorted_on = None

    ###
    # Intermediate fuzzing steps - these steps perform actions that should not affect the final sort result
    # Some steps are skipped because they encounter bugs that need to be fixed.
    ###

    # @rule(data=data())
    # @precondition(lambda self: self.df is not None)
    # def repartition_df(self, data):
    #     """Runs a repartitioning step"""
    #     num_partitions = data.draw(self.repartition_num_partitions_strategy, label="Number of partitions for repartitioning")
    #     self.df = self.df.repartition(num_partitions)

    # # Repartitioning changes the ordering of the data, so we cannot sort after this step
    # self.sorted_on = None

    # @rule(data=data())
    # @precondition(lambda self: self.df is not None)
    # def filter_df(self, data):
    #     """Runs a filter on a simple equality predicate on a random column"""
    #     assert self.df is not None
    #     col_name_to_filter = data.draw(sampled_from(self.df.schema().column_names()), label="Column to filter on")
    #     col_daft_type = self.df.schema()[col_name_to_filter].daft_type

    #     # Logical types do not accept equality operators, but can be filtered on by themselves
    #     if col_daft_type == ExpressionType.logical():
    #         self.df = self.df.where(self.df[col_name_to_filter])
    #     # Reject if filtering on a null column - not a meaningful operation
    #     elif col_daft_type == ExpressionType.null():
    #         reject()
    #     else:
    #         filter_value = data.draw(generate_data(col_daft_type), label="Filter value")
    #         self.df = self.df.where(self.df[col_name_to_filter] == filter_value)

    # @rule(data=data())
    # @precondition(lambda self: self.df is not None)
    # def project_df(self, data):
    #     """Runs a projection on a random column, replacing it"""
    #     assert self.df is not None
    #     column_name = data.draw(sampled_from(self.df.schema().column_names()), label="Column to filter on")
    #     column_daft_type = self.df.schema()[column_name].daft_type
    #     type_to_op_mapping = {
    #         ExpressionType.string(): lambda e, other: e.str.concat(other),
    #         ExpressionType.integer(): lambda e, other: e + other,
    #         ExpressionType.float(): lambda e, other: e + other,
    #         ExpressionType.logical(): lambda e, other: e & other,
    #         ExpressionType.from_py_type(UserObject): lambda e, other: e.apply(
    #             lambda x: x.add(other) if x is not None else None, return_type=UserObject
    #         ),
    #         # No meaningful binary operations supported for these yet
    #         ExpressionType.date(): lambda e, other: e.dt.year(),
    #         ExpressionType.bytes(): lambda e, other: e,
    #         ExpressionType.null(): lambda e, other: e,
    #     }
    #     op = type_to_op_mapping[column_daft_type]
    #     other_binary_value = data.draw(generate_data(column_daft_type), label="Binary *other* value")
    #     self.df = self.df.with_column(column_name, op(self.df[column_name], other_binary_value))


TestDataframeSortStateMachine = DataframeSortStateMachine.TestCase