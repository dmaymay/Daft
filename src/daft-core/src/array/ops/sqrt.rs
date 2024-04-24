use crate::{
    array::DataArray,
    datatypes::{
        Float32Array, Float32Type, Float64Array, Float64Type, Int16Array, Int32Array, Int64Array,
        Int8Array, UInt16Array, UInt32Array, UInt64Array, UInt8Array,
    },
    DataType,
};
use common_error::DaftResult;

impl Int8Array {
    pub fn sqrt(&self) -> DaftResult<Float32Array> {
        self.cast(&DataType::Float32)?
            .downcast::<DataArray<Float32Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl Int16Array {
    pub fn sqrt(&self) -> DaftResult<Float32Array> {
        self.cast(&DataType::Float32)?
            .downcast::<DataArray<Float32Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl UInt8Array {
    pub fn sqrt(&self) -> DaftResult<Float32Array> {
        self.cast(&DataType::Float32)?
            .downcast::<DataArray<Float32Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl UInt16Array {
    pub fn sqrt(&self) -> DaftResult<Float32Array> {
        self.cast(&DataType::Float32)?
            .downcast::<DataArray<Float32Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl Int32Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl Int64Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl UInt32Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl UInt64Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl Float32Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}

impl Float64Array {
    pub fn sqrt(&self) -> DaftResult<Float64Array> {
        self.cast(&DataType::Float64)?
            .downcast::<DataArray<Float64Type>>()?
            .apply(|v| v.sqrt())
    }
}
