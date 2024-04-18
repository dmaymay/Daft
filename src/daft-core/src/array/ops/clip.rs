use crate::datatypes::{
    Float32Array, Float64Array, Int128Array, Int16Array, Int32Array, Int64Array, Int8Array,
    UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};
use common_error::DaftResult;

macro_rules! impl_clip_for_numeric_array {
    ($array_type:ident, $type:ty, float) => {
        impl $array_type {
            pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<Self>
            where
                T: Into<f64> + Copy,
            {
                self.apply(|v| {
                    if v.is_nan() {
                        return v;
                    }
                    let lower_bound = lower.map_or(<$type>::MIN, |l| l.into() as $type);
                    let upper_bound = upper.map_or(<$type>::MAX, |u| u.into() as $type);
                    v.clamp(lower_bound, upper_bound)
                })
            }
        }
    };

    ($array_type:ident, $type:ty, int) => {
        impl $array_type {
            pub fn clip<T>(&self, lower: Option<T>, upper: Option<T>) -> DaftResult<Self>
            where
                T: Into<f64> + Copy,
            {
                self.apply(|v| {
                    let lower_bound = lower.map_or(<$type>::MIN, |l| l.into() as $type);
                    let upper_bound = upper.map_or(<$type>::MAX, |u| u.into() as $type);
                    v.clamp(lower_bound, upper_bound)
                })
            }
        }
    };
}

impl_clip_for_numeric_array!(Int8Array, i8, int);
impl_clip_for_numeric_array!(Int16Array, i16, int);
impl_clip_for_numeric_array!(Int32Array, i32, int);
impl_clip_for_numeric_array!(Int64Array, i64, int);
impl_clip_for_numeric_array!(Int128Array, i128, int);

impl_clip_for_numeric_array!(UInt8Array, u8, int);
impl_clip_for_numeric_array!(UInt16Array, u16, int);
impl_clip_for_numeric_array!(UInt32Array, u32, int);
impl_clip_for_numeric_array!(UInt64Array, u64, int);

impl_clip_for_numeric_array!(Float32Array, f32, float);
impl_clip_for_numeric_array!(Float64Array, f64, float);
