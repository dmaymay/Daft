use crate::{array::DataArray, datatypes::DaftNumericType};
use common_error::DaftResult;
use num_traits::{FromPrimitive, ToPrimitive};

impl<T: DaftNumericType> DataArray<T>
where
    T::Native: ToPrimitive + FromPrimitive,
{
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let v_as_f64 = v.to_f64().unwrap_or_default();
            let lower_bound = lower.unwrap_or(std::f64::MIN);
            let upper_bound = upper.unwrap_or(std::f64::MAX);
            let clamped_f64 = v_as_f64.clamp(lower_bound, upper_bound);
            FromPrimitive::from_f64(clamped_f64).unwrap_or(v)
        })
    }
}
