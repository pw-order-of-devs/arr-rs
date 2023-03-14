/// Base numeric type for array
pub trait Numeric: Copy + Clone + PartialEq + PartialOrd +
std::ops::Add<Self, Output=Self> + std::ops::Sub<Self, Output=Self> +
std::ops::Mul<Self, Output=Self> + std::ops::Div<Self, Output=Self> {
    /// Zero constant value
    const ZERO: Self;
    /// One constant value
    const ONE: Self;
}

macro_rules! impl_zero_one_numeric {
    ($t:ty, $zero:expr, $one:expr) => {
        impl Numeric for $t {
            const ZERO: Self = $zero;
            const ONE: Self = $one;
        }
    };
}

impl_zero_one_numeric!(f32, 0.0, 1.0);
impl_zero_one_numeric!(f64, 0.0, 1.0);
impl_zero_one_numeric!(i8, 0, 1);
impl_zero_one_numeric!(i16, 0, 1);
impl_zero_one_numeric!(i32, 0, 1);
impl_zero_one_numeric!(i64, 0, 1);
impl_zero_one_numeric!(u8, 0, 1);
impl_zero_one_numeric!(u16, 0, 1);
impl_zero_one_numeric!(u32, 0, 1);
impl_zero_one_numeric!(u64, 0, 1);
