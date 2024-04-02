use crate::datatypes::{
    Float32Array, Float64Array, Int128Array, Int16Array, Int32Array, Int64Array, Int8Array,
    UInt16Array, UInt32Array, UInt64Array, UInt8Array,
};

use common_error::DaftResult;

impl Float32Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            if v.is_nan() {
                return v;
            }
            let lower_bound = lower.unwrap_or(f32::MIN.into()) as f32;
            let upper_bound = upper.unwrap_or(f32::MAX.into()) as f32;
            v.max(lower_bound).min(upper_bound)
        })
    }
}

impl Float64Array {
    pub fn clip(&self, lower: Option<f64>, upper: Option<f64>) -> DaftResult<Self> {
        self.apply(|v| {
            if v.is_nan() {
                return v;
            }
            let lower_bound = lower.unwrap_or(f64::MIN);
            let upper_bound = upper.unwrap_or(f64::MAX);
            v.max(lower_bound).min(upper_bound)
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
