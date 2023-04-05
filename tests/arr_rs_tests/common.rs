use arr_rs::prelude::*;

pub(crate) fn test_runner<N: Numeric>(
    arg1: &Array<N>,
    arg2: &Array<N>,
) {
    assert_eq!(arg1.get_shape(), arg2.get_shape(), "Shapes don't match");
    assert_eq!(format!("{arg1:.4}"), format!("{arg2:.4}"), "Arrays don't match");
}
