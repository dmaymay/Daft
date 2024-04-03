use crate::datatypes::{
    DaftNumericType, Int128Array, Int16Array, Int32Array, Int64Array, Int8Array, UInt16Array,
    UInt32Array, UInt64Array, UInt8Array,
};

use crate::{array::DataArray, datatypes::DaftFloatType};
use num_traits::{Float, NumCast};

use common_error::DaftResult;

impl<T: DaftFloatType> DataArray<T>
where
    T: DaftNumericType,
    T::Native: Float + NumCast,
{
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            if v.is_nan() {
                return v;
            }
            let lower_bound: Option<T::Native> = lower.and_then(NumCast::from);
            let upper_bound: Option<T::Native> = upper.and_then(NumCast::from);
            upper_bound.map_or(lower_bound.map_or(v, |lb| v.max(lb)), |ub| v.min(ub))
        })
    }
}

impl Int8Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(i8::MIN, |l| l as i8);
            let upper_bound = upper.map_or(i8::MAX, |u| u as i8);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl Int16Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(i16::MIN, |l| l as i16);
            let upper_bound = upper.map_or(i16::MAX, |u| u as i16);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl Int32Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(i32::MIN, |l| l as i32);
            let upper_bound = upper.map_or(i32::MAX, |u| u as i32);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl Int64Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(i64::MIN, |l| l as i64);
            let upper_bound = upper.map_or(i64::MAX, |u| u as i64);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl Int128Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(i128::MIN, |l| l as i128);
            let upper_bound = upper.map_or(i128::MAX, |u| u as i128);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl UInt8Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(u8::MIN, |l| l as u8);
            let upper_bound = upper.map_or(u8::MAX, |u| u as u8);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl UInt16Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(u16::MIN, |l| l as u16);
            let upper_bound = upper.map_or(u16::MAX, |u| u as u16);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl UInt32Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(u32::MIN, |l| l as u32);
            let upper_bound = upper.map_or(u32::MAX, |u| u as u32);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl UInt64Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            let lower_bound = lower.map_or(u64::MIN, |l| l as u64);
            let upper_bound = upper.map_or(u64::MAX, |u| u as u64);
            v.max(lower_bound).min(upper_bound)
        })
    }
}

/* enum ClippedIntegerResult {
    Int8Array(Int8Array),
    Int16Array(Int16Array),
    Int32Array(Int32Array),
    Int64Array(Int64Array),
    UInt8Array(UInt8Array),
    UInt16Array(UInt16Array),
    UInt32Array(UInt32Array),
    UInt64Array(UInt64Array),
    Float64Array(Float64Array),
}

impl<T: DaftIntegerType> DataArray<T>
where
    T: DaftNumericType,
    T::Native: NumCast + Copy + Ord + Bounded,
{
    pub fn clip_integer(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<ClippedIntegerResult> {
        if lower.map(|l| l.fract() == 0.0).unwrap_or(true) && upper.map(|u| u.fract() == 0.0).unwrap_or(true) {
            // Both bounds are whole numbers
            let result = self.apply(|v| {
                let lower_bound = lower.map_or(T::Native::min_value(), |l| NumCast::from(l).unwrap_or(T::Native::min_value()));
                let upper_bound = upper.map_or(T::Native::max_value(), |u| NumCast::from(u).unwrap_or(T::Native::max_value()));
                v.max(lower_bound).min(upper_bound)
            })?;
            match self.data_type() {
                DataType::Int8 => Ok(ClippedIntegerResult::Int8Array(result)),
                DataType::Int16 => Ok(ClippedIntegerResult::Int16Array(result)),
                DataType::Int32 => Ok(ClippedIntegerResult::Int32Array(result)),
                DataType::Int64 => Ok(ClippedIntegerResult::Int64Array(result)),
                DataType::UInt8 => Ok(ClippedIntegerResult::UInt8Array(result)),
                DataType::UInt16 => Ok(ClippedIntegerResult::UInt16Array(result)),
                DataType::UInt32 => Ok(ClippedIntegerResult::UInt32Array(result)),
                DataType::UInt64 => Ok(ClippedIntegerResult::UInt64Array(result)),
                _ => unreachable!(),
            }
        } else {
            // At least one bound is fractional
            let result = self.apply(|v| {
                let v_f64 = NumCast::from(v).unwrap_or(0.0f64);
                let lower_bound = lower.unwrap_or(f64::MIN);
                let upper_bound = upper.unwrap_or(f64::MAX);
                v_f64.max(lower_bound).min(upper_bound)
            })?;
            Ok(ClippedIntegerResult::Float64Array(result))
        }
    }
}
 */
