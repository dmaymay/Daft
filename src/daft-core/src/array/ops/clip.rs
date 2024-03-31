use crate::{array::DataArray, datatypes::DaftNumericType};
use common_error::DaftResult;
use num_traits::{FromPrimitive, ToPrimitive};

impl<T: DaftNumericType> DataArray<T>
where
    T::Native: ToPrimitive + FromPrimitive,
{
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        // Early check for bounds logic.
        if let (Some(lower_bound), Some(upper_bound)) = (lower, upper) {
            if lower_bound > upper_bound {
                // If lower_bound is greater than upper_bound, set all values to upper_bound.
                return self.apply(|_| FromPrimitive::from_f64(upper_bound).unwrap_or_default());
            }
        }

        self.apply(|v| {
            // Convert v to f64 for comparison.
            let v_as_f64 = v.to_f64().unwrap_or_default();

            let clipped_value = if let Some(lower_bound) = lower {
                if v_as_f64 < lower_bound {
                    FromPrimitive::from_f64(lower_bound).unwrap_or(v)
                } else {
                    v
                }
            } else {
                v
            };

            if let Some(upper_bound) = upper {
                if v_as_f64 > upper_bound {
                    FromPrimitive::from_f64(upper_bound).unwrap_or(clipped_value)
                } else {
                    clipped_value
                }
            } else {
                clipped_value
            }
        })
    }
}
