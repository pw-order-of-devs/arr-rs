use rstest::rstest;
use arr_rs::prelude::*;

#[rstest(
array, expected,
case(array!(f64, [[1, 2], [3, 4]]), Ok(vec![array_flat!(f64, 5.379562043795623, -0.37956204379562025).unwrap()])),
case(array!(f64, [[1, 4], [3, 2]]), Ok(vec![array_flat!(f64, 4.6, -1.6).unwrap()])),
case(array!(f64, [[2., -1., 0.], [-1., 2., -1.], [0., -1., 2.]]), Ok(vec![array_flat!(f64, 2.8, 2.3428571428571425, 0.8571428571428569).unwrap()])),
case(array!(f64, [[12., -51., 4.], [6., 167., -68.], [-4., 24., -41.]]), Ok(vec![array_flat!(f64, 156.20350762022625, -35.473244608879945, 17.26973698865371).unwrap()])),
case(array!(f64, [1, 4, 3]), Err(ArrayError::UnsupportedDimension { supported: vec![0, 1] })),
case(array!(f64, [[1, 4, 3], [3, 2, 4]]), Err(ArrayError::MustBeEqual { value1: "2".to_string(), value2: "3".to_string() })),
)] fn test_linalg_eigvals(array: Result<Array<f64>, ArrayError>, expected: Result<Vec<Array<f64>>, ArrayError>) {
    assert_eq!(expected, array.eigvals())
}

#[rstest(
array, expected,
case(array!(f64, [[1, 2], [3, 4]]), Ok(vec![(
array_flat!(f64, 5.379562043795623, -0.37956204379562025).unwrap(),
array!(f64, [[-0.41637599816479626, 0.8265490533652102], [-0.9091925143512071, -0.5628646927824439]]).unwrap(),
)])),
case(array!(f64, [[1, 4], [3, 2]]), Ok(vec![(
array_flat!(f64, 4.6, -1.6).unwrap(),
array!(f64, [[std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2], [std::f64::consts::FRAC_1_SQRT_2, std::f64::consts::FRAC_1_SQRT_2]]).unwrap(),
)])),
case(array!(f64, [[2., -1., 0.], [-1., 2., -1.], [0., -1., 2.]]), Ok(vec![(
array_flat!(f64, 2.8, 2.3428571428571425, 0.8571428571428569).unwrap(),
array!(f64, [[-0.16222142113076257, -0.3458744383324873, -0.4908149932527664], [-0.9733285267845752, -0.8722051053601856, -0.7198619901040574], [-0.16222142113076266, -0.34587443833248765, -0.49081499325276634]]).unwrap(),
)])),
case(array!(f64, [[12., -51., 4.], [6., 167., -68.], [-4., 24., -41.]]), Ok(vec![(
array_flat!(f64, 156.20350762022625, -35.473244608879945, 17.26973698865371).unwrap(),
array!(f64, [[0.3274744043621118, 0.2741043942532785, -0.9929229419852038], [-0.9370666300603048, 0.3040145084128465, 0.08536232642252124], [-0.12110592601150529, 0.9123825731158716, 0.08256696983166067]]).unwrap(),
)])),
case(array!(f64, [1, 4, 3]), Err(ArrayError::UnsupportedDimension { supported: vec![0, 1] })),
case(array!(f64, [[1, 4, 3], [3, 2, 4]]), Err(ArrayError::MustBeEqual { value1: "2".to_string(), value2: "3".to_string() })),
)] fn test_linalg_eig(array: Result<Array<f64>, ArrayError>, expected: EigenResult<f64>) {
    assert_eq!(expected, array.eig())
}
