use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Create functions
pub trait ArrayCreateFrom<N: Numeric> where Array<N>: Sized + Clone {

    // matrices

    /// Extract a diagonal or construct a diagonal array
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = array!(i32, [[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr = Array::diag(&array!(i32, [1, 2, 3, 4]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected = array!(i32, [1, 2, 3, 4]).unwrap();
    /// let arr = Array::diag(&array!(i32, [[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected = array!(i32, [0, 0, 0]).unwrap();
    /// let arr = Array::diag(&array!(i32, [[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap(), Some(1)).unwrap();
    /// assert_eq!(expected, arr);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn diag(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Construct a diagonal array for flattened input
    ///
    /// # Arguments
    ///
    /// * `data` - input array
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = array!(i32, [[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr = Array::diagflat(&array!(i32, [1, 2, 3, 4]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected = array!(i32, [[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr = Array::diagflat(&array!(i32, [[1, 2], [3, 4]]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn diagflat(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return a copy of an array with elements above the k-th diagonal zeroed.
    /// For arrays with ndim exceeding 2, tril will apply to the final two axes.
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 1, 8).reshape(&[2, 4]).unwrap();
    /// let expected = array!(i32, [[1, 0, 0, 0], [5, 6, 0, 0]]).unwrap();
    /// assert_eq!(expected, arr.tril(None).unwrap());
    ///
    /// let arr = array_arange!(i32, 1, 8).reshape(&[2, 2, 2]).unwrap();
    /// let expected = array!(i32, [[[1, 0], [3, 4]], [[5, 0], [7, 8]]]).unwrap();
    /// assert_eq!(expected, arr.tril(None).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn tril(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return a copy of an array with elements below the k-th diagonal zeroed.
    /// For arrays with ndim exceeding 2, triu will apply to the final two axes.
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(i32, 1, 8).reshape(&[2, 4]).unwrap();
    /// let expected = array!(i32, [[1, 2, 3, 4], [0, 6, 7, 8]]).unwrap();
    /// assert_eq!(expected, arr.triu(None).unwrap());
    ///
    /// let arr = array_arange!(i32, 1, 8).reshape(&[2, 2, 2]).unwrap();
    /// let expected = array!(i32, [[[1, 2], [0, 4]], [[5, 6], [0, 8]]]).unwrap();
    /// assert_eq!(expected, arr.triu(None).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn triu(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Generate a Vandermonde matrix
    ///
    /// # Arguments
    ///
    /// * `n` - number of columns in the output. optional, by default square array is returned
    /// * `increasing` - order of the powers of the columns. optional, defaults to false
    /// if true, the powers increase from left to right, if false, they are reversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(i32, [1, 2, 3, 4]).unwrap();
    /// let expected = array!(i32, [[1, 1, 1, 1], [8, 4, 2, 1], [27, 9, 3, 1], [64, 16, 4, 1]]).unwrap();
    /// assert_eq!(expected, arr.vander(None, Some(false)).unwrap());
    ///
    /// let arr = array!(i32, [1, 2, 3, 4]).unwrap();
    /// let expected = array!(i32, [[1, 1, 1, 1], [1, 2, 4, 8], [1, 3, 9, 27], [1, 4, 16, 64]]).unwrap();
    /// assert_eq!(expected, arr.vander(None, Some(true)).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn vander(&self, n: Option<usize>, increasing: Option<bool>) -> Result<Self, ArrayError> where Self: Sized;
}

impl <N: Numeric> ArrayCreateFrom<N> for Array<N> {

    // ==== matrices

    fn diag(&self, k: Option<isize>) -> Result<Self, ArrayError> {

        fn diag_1d<N: Numeric>(data: &Array<N>, k: isize) -> Result<Array<N>, ArrayError> {
            let size = data.get_shape()?[0];
            let abs_k = k.unsigned_abs();
            let new_shape = vec![size + abs_k, size + abs_k];
            let data_elements = data.get_elements()?;
            let elements = (0..new_shape[0] * new_shape[1])
                .map(|idx| {
                    let (i, j) = (idx / new_shape[1], idx % new_shape[1]);
                    if k >= 0 && j == i + k.to_usize() {
                        if i < size { data_elements[i] }
                        else { N::zero() }
                    } else if k < 0 && i == j + abs_k {
                        if j < size { data_elements[j] }
                        else { N::zero() }
                    } else {
                        N::zero()
                    }
                })
                .collect();

            Array::new(elements, new_shape)
        }

        fn diag_2d<N: Numeric>(data: &Array<N>, k: isize) -> Result<Array<N>, ArrayError> {
            let rows = data.get_shape()?[0];
            let cols = data.get_shape()?[1];
            let (start_row, start_col) =
                if k >= 0 { (0, k.to_usize()) }
                else { ((-k).to_usize(), 0) };

            let data_elements = data.get_elements()?;
            let elements = (start_row..rows)
                .zip(start_col..cols)
                .map(|(i, j)| data_elements[i * cols + j])
                .collect::<Vec<N>>();

            Array::new(elements.clone(), vec![elements.len()])
        }

        self.is_dim_supported(&[1, 2])?;

        let k = k.unwrap_or(0);
        if self.ndim()? == 1 { diag_1d(self, k) }
        else { diag_2d(self, k) }
    }

    fn diagflat(&self, k: Option<isize>) -> Result<Self, ArrayError> {
        self.ravel()?.diag(k)
    }

    fn tril(&self, k: Option<isize>) -> Result<Self, ArrayError> {
        let k = k.unwrap_or(0);
        self.apply_triangular(k, |j, i, k| j > i + k)
    }

    fn triu(&self, k: Option<isize>) -> Result<Self, ArrayError> {
        let k = k.unwrap_or(0);
        self.apply_triangular(k, |j, i, k| j < i + k)
    }

    fn vander(&self, n: Option<usize>, increasing: Option<bool>) -> Result<Self, ArrayError> {
        self.is_dim_supported(&[1])?;

        let size = self.shape[0];
        let increasing = increasing.unwrap_or(false);
        let n_columns = n.unwrap_or(size);
        let mut elements = Vec::with_capacity(size * n_columns);

        for item in self {
            for i in 0..n_columns {
                let power = if increasing { i } else { n_columns - i - 1 }.to_f64();
                elements.push(N::from(item.to_f64().powf(power)));
            }
        }

        Self::new(elements, vec![size, n_columns])
    }
}

impl <N: Numeric> Array<N> {

    fn apply_triangular<F>(&self, k: isize, compare: F) -> Result<Self, ArrayError>
        where F: Fn(isize, isize, isize) -> bool {
        let last_dim = self.shape.len() - 1;
        let second_last_dim = self.shape.len() - 2;
        let chunk_size = self.shape[last_dim] * self.shape[second_last_dim];

        let elements = self.elements
            .chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(idx, &value)| {
                        let i = (idx / self.shape[last_dim]) % self.shape[second_last_dim];
                        let j = idx % self.shape[last_dim];
                        if compare(j.to_isize(), i.to_isize(), k) { N::zero() } else { value }
                    })
            })
            .collect();

        Self::new(elements, self.shape.clone())
    }
}
