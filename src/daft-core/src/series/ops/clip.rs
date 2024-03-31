use crate::datatypes::DataType;
use crate::series::Series;
use common_error::DaftError;
use common_error::DaftResult;

impl Series {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Series> {
        use crate::series::array_impl::IntoSeries;
        use DataType::*;
        match self.data_type() {
            UInt8 => Ok(self.u8().unwrap().clip(lower, upper)?.into_series()),
            UInt16 => Ok(self.u16().unwrap().clip(lower, upper)?.into_series()),
            UInt32 => Ok(self.u32().unwrap().clip(lower, upper)?.into_series()),
            UInt64 => Ok(self.u64().unwrap().clip(lower, upper)?.into_series()),
            Int8 => Ok(self.i8().unwrap().clip(lower, upper)?.into_series()),
            Int16 => Ok(self.i16().unwrap().clip(lower, upper)?.into_series()),
            Int32 => Ok(self.i32().unwrap().clip(lower, upper)?.into_series()),
            Int64 => Ok(self.i64().unwrap().clip(lower, upper)?.into_series()),
            Float32 => Ok(self.f32().unwrap().clip(lower, upper)?.into_series()),
            Float64 => Ok(self.f64().unwrap().clip(lower, upper)?.into_series()),
            dt => Err(DaftError::TypeError(format!(
                "clip not implemented for {}",
                dt
            ))),
        }
    }
}
