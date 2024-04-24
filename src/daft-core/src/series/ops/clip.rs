use crate::array::ops::clip::NumericArray;
use crate::datatypes::DataType;
use crate::series::Series;
use common_error::DaftError;
use common_error::DaftResult;

// Assuming IntoSeries is a trait that defines into_series() for DataArray types.
trait IntoSeries {
    fn into_series(self) -> DaftResult<Series>;
}

impl NumericArray {
    pub fn into_series(self) -> DaftResult<Series> {
        use crate::series::array_impl::IntoSeries;

        match self {
            NumericArray::Int8(result) => Ok(result?.into_series()),
            NumericArray::Int16(result) => Ok(result?.into_series()),
            NumericArray::Int32(result) => Ok(result?.into_series()),
            NumericArray::Int64(result) => Ok(result?.into_series()),
            NumericArray::Int128(result) => Ok(result?.into_series()),
            NumericArray::UInt8(result) => Ok(result?.into_series()),
            NumericArray::UInt16(result) => Ok(result?.into_series()),
            NumericArray::UInt32(result) => Ok(result?.into_series()),
            NumericArray::UInt64(result) => Ok(result?.into_series()),
            NumericArray::F32(result) => Ok(result?.into_series()),
            NumericArray::F64(result) => Ok(result?.into_series()),
        }
    }
}

impl Series {
    pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<Series>
    where
        T: Into<f64> + Copy,
    {
        match self.data_type() {
            DataType::UInt8 => self.u8().unwrap().clip(lower, upper)?.into_series(),
            DataType::UInt16 => self.u16().unwrap().clip(lower, upper)?.into_series(),
            DataType::UInt32 => self.u32().unwrap().clip(lower, upper)?.into_series(),
            DataType::UInt64 => self.u64().unwrap().clip(lower, upper)?.into_series(),
            DataType::Int8 => self.i8().unwrap().clip(lower, upper)?.into_series(),
            DataType::Int16 => self.i16().unwrap().clip(lower, upper)?.into_series(),
            DataType::Int32 => self.i32().unwrap().clip(lower, upper)?.into_series(),
            DataType::Int64 => self.i64().unwrap().clip(lower, upper)?.into_series(),
            DataType::Float32 => self.f32().unwrap().clip(lower, upper)?.into_series(),
            DataType::Float64 => self.f64().unwrap().clip(lower, upper)?.into_series(),
            dt => Err(DaftError::TypeError(format!(
                "clip not implemented for {}",
                dt
            ))),
        }
    }
}
