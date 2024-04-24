use crate::datatypes::{
    DataArray, DataType, Float32Array, Float32Type, Float64Array, Float64Type, Int128Array,
    Int16Array, Int32Array, Int64Array, Int8Array, UInt16Array, UInt32Array, UInt64Array,
    UInt8Array,
};
use common_error::DaftResult;
use num_traits::ToPrimitive;

pub enum NumericArray {
    Int8(DaftResult<Int8Array>),
    Int16(DaftResult<Int16Array>),
    Int32(DaftResult<Int32Array>),
    Int64(DaftResult<Int64Array>),
    Int128(DaftResult<Int128Array>),
    UInt8(DaftResult<UInt8Array>),
    UInt16(DaftResult<UInt16Array>),
    UInt32(DaftResult<UInt32Array>),
    UInt64(DaftResult<UInt64Array>),
    F64(DaftResult<Float64Array>),
    F32(DaftResult<Float32Array>),
}

macro_rules! impl_clip_for_numeric_array {
    ($array_type:ident, $type:ty, $variant:ident, float) => {
        impl $array_type {
            pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<NumericArray>
            where
                T: Into<f64> + Copy,
            {
                let result = self.apply(|v| {
                    if v.is_nan() {
                        return v;
                    }
                    let lower_bound = lower.map_or(<$type>::MIN, |l| l.into() as $type);
                    let upper_bound = upper.map_or(<$type>::MAX, |u| u.into() as $type);
                    v.clamp(lower_bound, upper_bound)
                });
                Ok(NumericArray::$variant(DaftResult::Ok(result?)))
            }
        }
    };

    ($array_type:ident, $type:ty, $variant:ident, $convert:ident, int_or_f64) => {
        impl $array_type {
            pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<NumericArray>
            where
                T: Into<f64> + Copy,
            {
                let mut float_bounds = false;

                if let Some(l) = lower {
                    let l_f64: f64 = l.into();
                    if l_f64.fract() != 0.0 {
                        float_bounds = true;
                    }
                }
                if let Some(u) = upper {
                    let u_f64: f64 = u.into();
                    if u_f64.fract() != 0.0 {
                        float_bounds = true;
                    }
                }

                if float_bounds {
                    let float_array = self.cast(&DataType::Float64)?;
                    let result = float_array
                        .downcast::<DataArray<Float64Type>>()?
                        .apply(|v| {
                            let v_as_float = v.to_f64().unwrap_or_default();
                            let float_lower = lower.map_or(f64::MIN, |l| l.into() as f64);
                            let float_upper = upper.map_or(f64::MAX, |u| u.into() as f64);
                            v_as_float.clamp(float_lower as f64, float_upper as f64)
                        })?;
                    Ok(NumericArray::$convert(DaftResult::Ok(result.clone())))
                } else {
                    let result = self.apply(|v| {
                        let lower_bound = lower.map_or(<$type>::MIN, |l| l.into() as $type);
                        let upper_bound = upper.map_or(<$type>::MAX, |u| u.into() as $type);
                        v.clamp(lower_bound, upper_bound)
                    })?;
                    Ok(NumericArray::$variant(DaftResult::Ok(result)))
                }
            }
        }
    };

    ($array_type:ident, $type:ty, $variant:ident, $convert:ident, int_or_f32) => {
        impl $array_type {
            pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<NumericArray>
            where
                T: Into<f64> + Copy,
            {
                let mut float_bounds = false;

                if let Some(l) = lower {
                    let l_f64: f64 = l.into();
                    if l_f64.fract() != 0.0 {
                        float_bounds = true;
                    }
                }
                if let Some(u) = upper {
                    let u_f64: f64 = u.into();
                    if u_f64.fract() != 0.0 {
                        float_bounds = true;
                    }
                }

                if float_bounds {
                    let float_array = self.cast(&DataType::Float32)?;
                    let result = float_array
                        .downcast::<DataArray<Float32Type>>()?
                        .apply(|v| {
                            let v_as_float = v.to_f32().unwrap_or_default();
                            let float_lower = lower.map_or(f32::MIN, |l| l.into() as f32);
                            let float_upper = upper.map_or(f32::MAX, |u| u.into() as f32);
                            v_as_float.clamp(float_lower as f32, float_upper as f32)
                        })?;
                    Ok(NumericArray::$convert(DaftResult::Ok(result.clone())))
                } else {
                    let result = self.apply(|v| {
                        let lower_bound = lower.map_or(<$type>::MIN, |l| l.into() as $type);
                        let upper_bound = upper.map_or(<$type>::MAX, |u| u.into() as $type);
                        v.clamp(lower_bound, upper_bound)
                    })?;
                    Ok(NumericArray::$variant(DaftResult::Ok(result)))
                }
            }
        }
    };
}

impl_clip_for_numeric_array!(Int8Array, i8, Int8, F32, int_or_f32);
impl_clip_for_numeric_array!(Int16Array, i16, Int16, F32, int_or_f32);
impl_clip_for_numeric_array!(Int32Array, i32, Int32, F64, int_or_f64);
impl_clip_for_numeric_array!(Int64Array, i64, Int64, F64, int_or_f64);
impl_clip_for_numeric_array!(Int128Array, i128, Int128, F64, int_or_f64);
impl_clip_for_numeric_array!(UInt8Array, u8, UInt8, F32, int_or_f32);
impl_clip_for_numeric_array!(UInt16Array, u16, UInt16, F32, int_or_f32);
impl_clip_for_numeric_array!(UInt32Array, u32, UInt32, F64, int_or_f64);
impl_clip_for_numeric_array!(UInt64Array, u64, UInt64, F64, int_or_f64);

impl_clip_for_numeric_array!(Float32Array, f32, F32, float);
impl_clip_for_numeric_array!(Float64Array, f64, F64, float);
