use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Math Special functions
pub trait ArrayMathSpecial<N: NumericOps> where Self: Sized + Clone {

    /// Modified Bessel function of the first kind, order 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-2., 0., 3.5]);
    /// assert_eq!(Array::flat(vec![2.27958510662287, 0.9999999999999997, 7.378203432225479]), arr.i0());
    /// ```
    fn i0(&self) -> Result<Array<N>, ArrayError>;

    /// Return the normalized sinc function
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]);
    /// assert_eq!(Array::flat(vec![3.898_171_832_519_375_5e-17, 1.0, 3.898_171_832_519_375_5e-17]), arr.sinc());
    /// ```
    fn sinc(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArrayMathSpecial<N> for Array<N> {

    fn i0(&self) -> Result<Array<N>, ArrayError> {
        self.map(|&x| N::from(i0(x.to_f64())))
    }

    fn sinc(&self) -> Result<Array<N>, ArrayError> {
        self.map(|&x| {
            let y = std::f64::consts::PI *
                if x == N::zero() { 1.0e-20 }
                else { x.to_f64() };
            N::from(y.sin() / y)
        })
    }
}

impl <N: NumericOps> ArrayMathSpecial<N> for Result<Array<N>, ArrayError> {

    fn i0(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.i0()
    }

    fn sinc(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sinc()
    }
}

const VALUES_I0_A: &[f64] = &[
    -4.415_341_646_479_339E-18,
    3.330_794_518_822_238E-17,
    -2.431_279_846_547_954E-16,
    1.715_391_285_555_133E-15,
    -1.168_533_287_799_345E-14,
    7.676_185_498_604_935E-14,
    -4.856_446_783_111_929E-13,
    2.955_052_663_129_639E-12,
    -1.726_826_291_441_555E-11,
    9.675_809_035_373_236E-11,
    -5.189_795_601_635_262E-10,
    2.659_823_724_682_386E-9,
    -1.300_025_009_986_248E-8,
    6.046_995_022_541_918E-8,
    -2.670_793_853_940_611E-7,
    1.117_387_539_120_103E-6,
    -4.416_738_358_458_75E-6,
    1.644_844_807_072_889E-5,
    -5.754_195_010_082_103E-5,
    1.885_028_850_958_416E-4,
    -5.763_755_745_385_823E-4,
    1.639_475_616_941_335E-3,
    -4.324_309_995_050_575E-3,
    1.054_646_039_459_499E-2,
    -2.373_741_480_589_946E-2,
    4.930_528_423_967_07E-2,
    -9.490_109_704_804_764E-2,
    1.716_209_015_222_087E-1,
    -3.046_826_723_431_983E-1,
    6.767_952_744_094_76E-1,
];

const VALUES_I0_B: &[f64] = &[
    -7.233_180_487_874_753E-18,
    -4.830_504_485_944_182E-18,
    4.465_621_420_296_759E-17,
    3.461_222_867_697_461E-17,
    -2.827_623_980_516_583E-16,
    -3.425_485_619_677_219E-16,
    1.772_560_133_056_526E-15,
    3.811_680_669_352_622E-15,
    -9.554_846_698_828_307E-15,
    -4.150_569_347_287_222E-14,
    1.540_086_217_521_409E-14,
    3.852_778_382_742_142E-13,
    7.180_124_451_383_666E-13,
    -1.794_178_531_506_806E-12,
    -1.321_581_184_044_771E-11,
    -3.149_916_527_963_241E-11,
    1.188_914_710_784_643E-11,
    4.940_602_388_224_969E-10,
    3.396_232_025_708_386E-9,
    2.266_668_990_498_178E-8,
    2.048_918_589_469_063E-7,
    2.891_370_520_834_756E-6,
    6.889_758_346_916_823E-5,
    3.369_116_478_255_694E-3,
    8.044_904_110_141_088E-1,
];

fn chbevl(x: f64, vals: &[f64]) -> f64 {
    let mut b0 = vals[0];
    let mut b1 = 0.;

    let mut b2 = 0.;
    for val in vals.iter().skip(1) {
        b2 = b1;
        b1 = b0;
        b0 = x * b1 - b2 + val;
    }

    0.5 * (b0 - b2)
}

fn i0(x: f64) -> f64 {
    if x <= 8.0 { f64::exp(x) * chbevl(x / 2. - 2., VALUES_I0_A) }
    else { f64::exp(x) * chbevl(32. / x - 2., VALUES_I0_B) / f64::sqrt(x) }
}
