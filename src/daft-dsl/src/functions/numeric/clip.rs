use common_error::{DaftError, DaftResult};
use daft_core::{datatypes::Field, schema::Schema, series::Series};

use super::super::FunctionEvaluator;
use super::NumericExpr;
use crate::Expr;

use crate::functions::FunctionExpr;

pub(super) struct ClipEvaluator {}

impl FunctionEvaluator for ClipEvaluator {
    fn fn_name(&self) -> &'static str {
        "clip"
    }

    fn to_field(&self, inputs: &[Expr], schema: &Schema, _: &Expr) -> DaftResult<Field> {
        if inputs.len() != 1 {
            return Err(DaftError::SchemaMismatch(format!(
                "Expected 1 input arg, got {}",
                inputs.len()
            )));
        }
        let field = inputs.first().unwrap().to_field(schema)?;
        if !field.dtype.is_numeric() {
            return Err(DaftError::TypeError(format!(
                "Expected input to clip to be numeric, got {}",
                field.dtype
            )));
        }
        Ok(field)
    }

    fn evaluate(&self, inputs: &[Series], expr: &Expr) -> DaftResult<Series> {
        if inputs.len() != 1 {
            return Err(DaftError::SchemaMismatch(format!(
                "Expected 1 input arg for clip, got {}",
                inputs.len()
            )));
        }
        let (lower, upper) = match expr {
            Expr::Function {
                func: FunctionExpr::Numeric(NumericExpr::Clip(lower, upper)),
                inputs: _,
            } => (lower, upper),
            _ => panic!("Expected Clip Expr, got {:?}", expr),
        };
        // Apply clipping to the first (and only) series in inputs
        inputs.first().unwrap().clip(*lower, *upper)
    }
}
