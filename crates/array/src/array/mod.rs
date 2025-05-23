use crate::gpu_utils::*;
use crate::kernels::ScalarValue;
use crate::kernels::broadcast::Broadcast;
use crate::utils::ScalarArray;
use buffer::ArrowGpuBuffer;
use bytemuck::Pod;
use std::sync::Arc;
use std::{any::Any, fmt::Debug};

pub(crate) mod boolean_gpu;
pub mod buffer;
pub(crate) mod date32_gpu;
pub(crate) mod f32_gpu;
pub(crate) mod i16_gpu;
pub(crate) mod i32_gpu;
pub(crate) mod i8_gpu;
pub(crate) mod null_bit_buffer;
pub(crate) mod primitive_array_gpu;
pub mod types;
pub(crate) mod u16_gpu;
pub(crate) mod u32_gpu;
pub(crate) mod u8_gpu;

pub use boolean_gpu::BooleanArrayGPU;
pub use date32_gpu::Date32ArrayGPU;
pub use date32_gpu::Date32Type;
pub use f32_gpu::Float32ArrayGPU;
pub use i8_gpu::Int8ArrayGPU;
pub use i16_gpu::Int16ArrayGPU;
pub use i32_gpu::Int32ArrayGPU;
pub use null_bit_buffer::*;
pub use primitive_array_gpu::PrimitiveArrayGpu;
pub use u8_gpu::UInt8ArrayGPU;
pub use u16_gpu::UInt16ArrayGPU;
pub use u32_gpu::UInt32ArrayGPU;

/// Enum of apache arrow datatypes
#[derive(Debug)]
#[non_exhaustive]
pub enum ArrowType {
    BooleanType,
    Float32Type,
    UInt32Type,
    UInt16Type,
    UInt8Type,
    Int32Type,
    Int16Type,
    Int8Type,
    Date32Type,
}

/// Trait expressing a Rust type that has the same in-memory representation as Arrow.
pub trait RustNativeType: Pod + Debug + Default {}

impl RustNativeType for i32 {}
impl RustNativeType for i16 {}
impl RustNativeType for i8 {}
impl RustNativeType for f32 {}
impl RustNativeType for u32 {}
impl RustNativeType for u16 {}
impl RustNativeType for u8 {}

/// Helper trait for types backed by primitive values
pub trait ArrowPrimitiveType: Send + Sync {
    type NativeType: RustNativeType;
    const ITEM_SIZE: u64;
}

macro_rules! impl_primitive_type {
    ($primitive_type: ident, $t: ident, $size: expr) => {
        impl ArrowPrimitiveType for $primitive_type {
            type NativeType = $t;
            const ITEM_SIZE: u64 = $size;
        }
    };
}

impl_primitive_type!(f32, f32, 4);
impl_primitive_type!(u32, u32, 4);
impl_primitive_type!(u16, u16, 2);
impl_primitive_type!(u8, u8, 1);
impl_primitive_type!(i32, i32, 4);
impl_primitive_type!(i16, i16, 4);
impl_primitive_type!(i8, i8, 1);
impl_primitive_type!(Date32Type, i32, 4);

pub(crate) trait ArrowArray: Any + Sync + Send + Debug {
    fn as_any(&self) -> &dyn Any;
    fn get_data_type(&self) -> ArrowType;
    fn get_memory_used(&self) -> u64;
    fn get_gpu_device(&self) -> &GpuDevice;
    fn get_buffer(&self) -> &ArrowGpuBuffer;
    fn get_null_bit_buffer(&self) -> Option<&NullBitBufferGpu>;
}

/// Trait for utility functions implemented by all gpu arrow array
pub trait ArrayUtils {
    fn get_gpu_device(&self) -> Arc<GpuDevice>;
}

/// Enum of apache arrow array
#[derive(Debug)]
#[non_exhaustive]
pub enum ArrowArrayGPU {
    Float32ArrayGPU(Float32ArrayGPU),
    UInt32ArrayGPU(UInt32ArrayGPU),
    UInt16ArrayGPU(UInt16ArrayGPU),
    UInt8ArrayGPU(UInt8ArrayGPU),
    Int32ArrayGPU(Int32ArrayGPU),
    Int16ArrayGPU(Int16ArrayGPU),
    Int8ArrayGPU(Int8ArrayGPU),
    Date32ArrayGPU(Date32ArrayGPU),
    BooleanArrayGPU(BooleanArrayGPU),
}

impl ArrowArrayGPU {
    pub fn get_gpu_device(&self) -> Arc<GpuDevice> {
        match self {
            ArrowArrayGPU::Float32ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::UInt32ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::UInt16ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::UInt8ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::Int32ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::Int16ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::Int8ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::Date32ArrayGPU(x) => x.gpu_device.clone(),
            ArrowArrayGPU::BooleanArrayGPU(x) => x.gpu_device.clone(),
        }
    }

    pub fn get_dtype(&self) -> ArrowType {
        match self {
            ArrowArrayGPU::Float32ArrayGPU(_) => ArrowType::Float32Type,
            ArrowArrayGPU::UInt32ArrayGPU(_) => ArrowType::UInt32Type,
            ArrowArrayGPU::UInt16ArrayGPU(_) => ArrowType::UInt16Type,
            ArrowArrayGPU::UInt8ArrayGPU(_) => ArrowType::UInt8Type,
            ArrowArrayGPU::Int32ArrayGPU(_) => ArrowType::Int32Type,
            ArrowArrayGPU::Int16ArrayGPU(_) => ArrowType::Int16Type,
            ArrowArrayGPU::Int8ArrayGPU(_) => ArrowType::Int8Type,
            ArrowArrayGPU::Date32ArrayGPU(_) => ArrowType::Date32Type,
            ArrowArrayGPU::BooleanArrayGPU(_) => ArrowType::BooleanType,
        }
    }

    pub fn get_raw_values(&self) -> ScalarArray {
        match self {
            ArrowArrayGPU::Float32ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::UInt16ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::UInt32ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::UInt8ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::Int32ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::Int16ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::Int8ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::Date32ArrayGPU(x) => x.raw_values().unwrap().into(),
            ArrowArrayGPU::BooleanArrayGPU(x) => x.raw_values().unwrap().into(),
        }
    }

    pub fn clone_array(&self) -> ArrowArrayGPU {
        match self {
            ArrowArrayGPU::Float32ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::UInt32ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::UInt16ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::UInt8ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::Int32ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::Int16ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::Int8ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::Date32ArrayGPU(x) => x.clone_array().into(),
            ArrowArrayGPU::BooleanArrayGPU(_) => todo!(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ArrowArrayGPU::Float32ArrayGPU(x) => x.len,
            ArrowArrayGPU::UInt32ArrayGPU(x) => x.len,
            ArrowArrayGPU::UInt16ArrayGPU(x) => x.len,
            ArrowArrayGPU::UInt8ArrayGPU(x) => x.len,
            ArrowArrayGPU::Int32ArrayGPU(x) => x.len,
            ArrowArrayGPU::Int16ArrayGPU(x) => x.len,
            ArrowArrayGPU::Int8ArrayGPU(x) => x.len,
            ArrowArrayGPU::Date32ArrayGPU(x) => x.len,
            ArrowArrayGPU::BooleanArrayGPU(x) => x.len,
        }
    }
}

/// Broadcast a single scalar value across the entire array of length `len`
pub fn broadcast_dyn(value: ScalarValue, len: usize, device: Arc<GpuDevice>) -> ArrowArrayGPU {
    match value {
        ScalarValue::F32(x) => Float32ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::U32(x) => UInt32ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::U16(x) => UInt16ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::U8(x) => UInt8ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::I32(x) => Int32ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::I16(x) => Int16ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::I8(x) => Int8ArrayGPU::broadcast(x, len, device).into(),
        ScalarValue::BOOL(x) => BooleanArrayGPU::broadcast(x, len, device).into(),
    }
}

/// Adds a command to the pipeline to
/// broadcast a single scalar value across the entire array of length `len`
pub fn broadcast_op_dyn(
    value: ScalarValue,
    len: usize,
    pipeline: &mut ArrowComputePipeline,
) -> ArrowArrayGPU {
    match value {
        ScalarValue::F32(x) => Float32ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::U32(x) => UInt32ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::U16(x) => UInt16ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::U8(x) => UInt8ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::I32(x) => Int32ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::I16(x) => Int16ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::I8(x) => Int8ArrayGPU::broadcast_op(x, len, pipeline).into(),
        ScalarValue::BOOL(x) => BooleanArrayGPU::broadcast_op(x, len, pipeline).into(),
    }
}
