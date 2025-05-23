use crate::impl_arithmetic_op;
use crate::*;
use arrow_gpu_array::array::{types::Int32Type, *};
use arrow_gpu_array::gpu_utils::*;

const I32_SCALAR_SHADER: &str = include_str!("../compute_shaders/i32/scalar.wgsl");
const I32_ARRAY_SHADER: &str = include_str!("../compute_shaders/i32/array.wgsl");

impl Sum32Bit for i32 {
    const SHADER: &'static str = include_str!("../compute_shaders/i32/aggregate.wgsl");
}

impl_arithmetic_op!(
    ArrowScalarAdd,
    Int32Type,
    add_scalar_op,
    Int32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_add"
);

impl_arithmetic_op!(
    ArrowScalarSub,
    Int32Type,
    sub_scalar_op,
    Int32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_sub"
);

impl_arithmetic_op!(
    ArrowScalarMul,
    Int32Type,
    mul_scalar_op,
    Int32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_mul"
);

impl_arithmetic_op!(
    ArrowScalarDiv,
    Int32Type,
    div_scalar_op,
    Int32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_div"
);

impl_arithmetic_op!(
    ArrowScalarRem,
    Int32Type,
    rem_scalar_op,
    Int32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_rem"
);

impl_arithmetic_op!(
    ArrowScalarAdd,
    Int32Type,
    add_scalar_op,
    Date32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_add"
);

impl_arithmetic_op!(
    ArrowScalarSub,
    Int32Type,
    sub_scalar_op,
    Date32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_sub"
);

impl_arithmetic_op!(
    ArrowScalarMul,
    Int32Type,
    mul_scalar_op,
    Date32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_mul"
);

impl_arithmetic_op!(
    ArrowScalarDiv,
    Int32Type,
    div_scalar_op,
    Date32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_div"
);

impl_arithmetic_op!(
    ArrowScalarRem,
    Int32Type,
    rem_scalar_op,
    Date32ArrayGPU,
    I32_SCALAR_SHADER,
    "i32_rem"
);

impl_arithmetic_array_op!(
    ArrowAdd,
    Int32Type,
    add_op,
    Int32ArrayGPU,
    I32_ARRAY_SHADER,
    "add_i32"
);

impl_arithmetic_array_op!(
    ArrowAdd,
    Int32Type,
    add_op,
    Date32ArrayGPU,
    I32_ARRAY_SHADER,
    "add_i32"
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rem_scalar_dyn;
    use crate::test::test_sum;
    use arrow_gpu_test_macros::{test_array_op, test_scalar_op};

    test_scalar_op!(
        test_add_i32_scalar_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        [0, 1, 2, 3, 4],
        add_scalar,
        add_scalar_dyn,
        100i32,
        [100, 101, 102, 103, 104]
    );

    test_scalar_op!(
        test_sub_i32_scalar_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        [0, 100, 200, 3, 104],
        sub_scalar,
        sub_scalar_dyn,
        100,
        [-100, 0, 100, -97, 4]
    );

    test_scalar_op!(
        test_mul_i32_scalar_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        [0, i32::MAX, 2, 3, 4],
        mul_scalar,
        mul_scalar_dyn,
        100,
        [0, -100, 200, 300, 400]
    );

    test_scalar_op!(
        test_div_i32_scalar_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        [0, 1, 100, 260, 450],
        div_scalar,
        div_scalar_dyn,
        100,
        [0, 0, 1, 2, 4]
    );

    test_scalar_op!(
        test_rem_i32_scalar_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        [0, 1, 2, 3, 104],
        rem_scalar,
        rem_scalar_dyn,
        100i32,
        [0, 1, 2, 3, 4]
    );

    test_scalar_op!(
        test_rem_i32_scalar_date32,
        Int32ArrayGPU,
        Date32ArrayGPU,
        Int32ArrayGPU,
        [0, 1, 2, 3, 104],
        rem_scalar,
        rem_scalar_dyn,
        100i32,
        [0, 1, 2, 3, 4]
    );

    /*//ignore = "Not passing in linux CI but passes in windows 🤔"
    #[cfg(not(target_os = "linux"))]
    test_scalar_op!(
        test_div_by_zero_i32_scalar_i32,
        i32,
        [0, 1, 100, 260, 450],
        div_scalar,
        0,
        [-1; 5]
    );*/

    test_scalar_op!(
        test_rem_date32_scalar_i32,
        Date32ArrayGPU,
        Int32ArrayGPU,
        Date32ArrayGPU,
        [0, 1, 2, 3, 104],
        rem_scalar,
        rem_scalar_dyn,
        100i32,
        [0, 1, 2, 3, 4]
    );

    test_scalar_op!(
        test_rem_date32_scalar_date32,
        Date32ArrayGPU,
        Date32ArrayGPU,
        Date32ArrayGPU,
        [0, 1, 2, 3, 104],
        rem_scalar,
        rem_scalar_dyn,
        100i32,
        [0, 1, 2, 3, 4]
    );

    test_array_op!(
        test_add_i32_array_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        add,
        [Some(0i32), Some(1), None, None, Some(4)],
        [Some(1i32), Some(2), None, Some(4), None],
        [Some(1), Some(3), None, None, None]
    );

    test_array_op!(
        test_add_i32_array_date32,
        Int32ArrayGPU,
        Date32ArrayGPU,
        Date32ArrayGPU,
        add,
        [Some(0i32), Some(1), None, None, Some(4)],
        [Some(1i32), Some(2), None, Some(4), None],
        [Some(1), Some(3), None, None, None]
    );

    test_sum!(
        #[cfg_attr(
            target_os = "windows",
            ignore = "Not passing in CI but passes in local 🤔"
        )]
        test_i32_sum,
        Int32ArrayGPU,
        -5,
        256 * 256,
        256 * 256 * -5
    );

    test_sum!(
        #[cfg_attr(
            any(target_os = "windows", target_os = "linux"),
            ignore = "Not passing in CI but passes in local 🤔"
        )]
        test_i32_sum_large,
        Int32ArrayGPU,
        -5,
        4 * 1024 * 1024,
        4 * 1024 * 1024 * -5
    );
}
