use arrow_gpu_array::array::Date32Type;

use crate::{SwizzleType, merge::U32_MERGE_SHADER, put::U32_PUT_SHADER, take::U32_TAKE_SHADER};

impl SwizzleType for i32 {
    const MERGE_SHADER: &'static str = U32_MERGE_SHADER;
    const TAKE_SHADER: &'static str = U32_TAKE_SHADER;
    const PUT_SHADER: &'static str = U32_PUT_SHADER;
}

impl SwizzleType for Date32Type {
    const MERGE_SHADER: &'static str = U32_MERGE_SHADER;
    const TAKE_SHADER: &'static str = U32_TAKE_SHADER;
    const PUT_SHADER: &'static str = U32_PUT_SHADER;
}

#[cfg(test)]
mod test {
    use crate::*;
    use arrow_gpu_array::array::*;

    test_merge_op!(
        test_merge_i32_array_i32,
        Int32ArrayGPU,
        Int32ArrayGPU,
        Int32ArrayGPU,
        merge,
        merge_dyn,
        [
            Some(0),
            Some(1),
            None,
            None,
            Some(4),
            Some(4),
            Some(10),
            None,
            Some(50)
        ],
        [
            Some(1),
            Some(2),
            None,
            Some(4),
            None,
            None,
            Some(20),
            Some(30),
            None
        ],
        [
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
            None,
            None,
            Some(false),
        ],
        [
            Some(0),
            Some(1),
            None,
            Some(4),
            Some(4),
            None,
            None,
            None,
            None
        ]
    );

    test_merge_op!(
        test_merge_date32_array_date32,
        Date32ArrayGPU,
        Date32ArrayGPU,
        Date32ArrayGPU,
        merge,
        [
            Some(0),
            Some(1),
            None,
            None,
            Some(4),
            Some(4),
            Some(10),
            None,
            Some(50)
        ],
        [
            Some(1),
            Some(2),
            None,
            Some(4),
            None,
            None,
            Some(20),
            Some(30),
            None
        ],
        [
            Some(true),
            Some(true),
            Some(false),
            Some(false),
            Some(true),
            Some(false),
            None,
            None,
            Some(false),
        ],
        [
            Some(0),
            Some(1),
            None,
            Some(4),
            Some(4),
            None,
            None,
            None,
            None
        ]
    );

    test_take_op!(
        test_take_i32,
        Int32ArrayGPU,
        UInt32ArrayGPU,
        Int32ArrayGPU,
        take,
        take_dyn,
        [Some(0), Some(1), None, Some(3)],
        [0, 1, 2, 3, 0, 1, 2, 3],
        [
            Some(0),
            Some(1),
            None,
            Some(3),
            Some(0),
            Some(1),
            None,
            Some(3)
        ]
    );

    // TODO test for cases with null
    test_put_op!(
        test_put_f32,
        Float32ArrayGPU,
        put,
        put_dyn,
        [0.0, 1.0, 2.0, 3.0],
        [0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 3.0, 0.0],
        [0, 1, 2, 3],
        [1, 3, 5, 7],
        [0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 3.0, 3.0]
    );

    // TODO test for cases with null
    test_put_op!(
        test_put_i32,
        Int32ArrayGPU,
        put,
        put_dyn,
        [0, 1, 2, 3],
        [0, 0, 1, 0, 2, 0, 3, 0],
        [0, 1, 2, 3],
        [1, 3, 5, 7],
        [0, 0, 1, 1, 2, 2, 3, 3]
    );

    // TODO test for cases with null
    test_put_op!(
        test_put_date32,
        Date32ArrayGPU,
        put,
        put_dyn,
        [0, 1, 2, 3],
        [0, 0, 1, 0, 2, 0, 3, 0],
        [0, 1, 2, 3],
        [1, 3, 5, 7],
        [0, 0, 1, 1, 2, 2, 3, 3]
    );
}
