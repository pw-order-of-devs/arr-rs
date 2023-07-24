use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Axis functions
pub trait ArrayAxis<T: ArrayElement> where Array<T>: Sized + Clone {

    /// Applies given function along an axis
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which the function will be applied
    /// * `f` - function to apply
    ///
    fn apply_along_axis<F>(&self, axis: usize, f: F) -> Result<Array<T>, ArrayError>
        where F: FnMut(&Array<T>) -> Result<Array<T>, ArrayError>;

    /// Returns an array with axes transposed
    ///
    /// # Arguments
    ///
    /// * `axes` - if defined, it's a list of axes to be included in transposition
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// assert_eq!(array!([[1, 5], [2, 6], [3, 7], [4, 8]]), arr.transpose(None));
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(array!([[1, 3, 5, 7], [2, 4, 6, 8]]), arr.transpose(None));
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(array!([[1, 3, 5, 7], [2, 4, 6, 8]]), arr.transpose(Some(vec![1, 0])));
    /// ```
    fn transpose(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError>;

    /// Move axes of an array to new positions
    ///
    /// # Arguments
    ///
    /// * `source` - original positions of the axes to move. must be unique
    /// * `destination` - destination positions for each of the original axes. must be unique
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![4, 5, 3], arr.moveaxis(vec![0], vec![2]).get_shape().unwrap());
    /// assert_eq!(vec![5, 3, 4], arr.moveaxis(vec![2], vec![0]).get_shape().unwrap());
    /// ```
    fn moveaxis(&self, source: Vec<isize>, destination: Vec<isize>) -> Result<Array<T>, ArrayError>;

    /// Roll the specified axis backwards, until it lies in a given position
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis to be rolled
    /// * `start` - start position. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![4, 3, 5], arr.rollaxis(1, None).get_shape().unwrap());
    /// assert_eq!(vec![3, 5, 4], arr.rollaxis(2, Some(1)).get_shape().unwrap());
    /// ```
    fn rollaxis(&self, axis: isize, start: Option<isize>) -> Result<Array<T>, ArrayError>;

    /// Interchange two axes of an array
    ///
    /// # Arguments
    ///
    /// * `axis_1` - first axis
    /// * `axis_1` - second axis
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![5, 4, 3], arr.swapaxes(0, 2).get_shape().unwrap());
    /// assert_eq!(vec![3, 5, 4], arr.swapaxes(2, 1).get_shape().unwrap());
    /// ```
    fn swapaxes(&self, axis: isize, start: isize) -> Result<Array<T>, ArrayError>;

    /// Expand the shape of an array
    ///
    /// # Arguments
    ///
    /// * `axes` - position in the expanded axes where the new axis (or axes) is placed
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![1, 3, 4, 5], arr.expand_dims(vec![0]).get_shape().unwrap());
    /// assert_eq!(vec![3, 1, 4, 1, 5], arr.expand_dims(vec![1, 3]).get_shape().unwrap());
    /// ```
    fn expand_dims(&self, axes: Vec<isize>) -> Result<Array<T>, ArrayError>;

    /// Remove axes of length one from array
    ///
    /// # Arguments
    ///
    /// * `axes` - position of the 10-sized axes to remove. if None, all such axes will be removed
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![1, 3, 1, 4, 5]);
    /// assert_eq!(vec![3, 4, 5], arr.squeeze(None).get_shape().unwrap());
    /// assert_eq!(vec![3, 1, 4, 5], arr.squeeze(Some(vec![0])).get_shape().unwrap());
    /// assert_eq!(vec![1, 3, 4, 5], arr.squeeze(Some(vec![2])).get_shape().unwrap());
    /// ```
    fn squeeze(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayAxis<T> for Array<T> {

    fn apply_along_axis<F>(&self, axis: usize, mut f: F) -> Result<Array<T>, ArrayError>
        where F: FnMut(&Array<T>) -> Result<Array<T>, ArrayError> {
        let parts = self.get_shape()?.remove_at(axis).into_iter().product();
        let array = self.moveaxis(vec![axis as isize], vec![self.ndim()? as isize])?;
        let partial = array
            .ravel()
            .split(parts, None)?.into_iter()
            .map(|arr| f(&arr))
            .collect::<Vec<Result<Array<T>, _>>>()
            .has_error()?.into_iter()
            .map(|arr| arr.unwrap())
            .collect::<Vec<Array<T>>>();
        let partial_len = partial[0].len()?;
        let partial = partial.into_iter().flatten().collect::<Array<T>>();

        let new_shape = array.get_shape()?.update_at(self.ndim()? - 1, partial_len);
        let partial = partial.reshape(&new_shape);
        if axis == 0 { partial.rollaxis((self.ndim()? - 1) as isize, None) }
        else { partial.moveaxis(vec![axis as isize], vec![(self.ndim()? - 1) as isize]) }
    }

    fn transpose(&self, axes: Option<Vec<isize>>) -> Result<Self, ArrayError> {

        fn transpose_recursive<T: ArrayElement>(
            input: &[T], input_shape: &[usize],
            output: &mut [T], output_shape: &[usize],
            current_indices: &mut [usize], current_dim: usize,
            axes: &Option<Vec<usize>>) {
            if current_dim < input_shape.len() - 1 {
                (0 .. input_shape[current_dim]).for_each(|i| {
                    current_indices[current_dim] = i;
                    transpose_recursive(input, input_shape, output, output_shape, current_indices, current_dim + 1, axes);
                });
            } else {
                (0 .. input_shape[current_dim]).for_each(|i| {
                    current_indices[current_dim] = i;
                    let input_index = input_shape.iter().enumerate().fold(0, |acc, (dim, size)| { acc * size + current_indices[dim] });
                    let output_indices = match axes {
                        Some(ref axes) => axes.iter().map(|&ax| current_indices[ax]).collect::<Vec<usize>>(),
                        None => current_indices.iter().rev().cloned().collect::<Vec<usize>>(),
                    };
                    let output_index = output_shape.iter().enumerate().fold(0, |acc, (dim, size)| { acc * size + output_indices[dim] });
                    output[output_index] = input[input_index].clone();
                });
            }
        }

        let axes = axes.map(|axes| axes.iter()
            .map(|i| self.normalize_axis(*i))
            .collect::<Vec<usize>>());
        let mut new_elements = vec![T::zero(); self.elements.len()];
        let new_shape: Vec<usize> = match axes.clone() {
            Some(axes) => axes.into_iter().map(|ax| self.shape[ax]).collect(),
            None => self.shape.clone().into_iter().rev().collect(),
        };

        transpose_recursive(
            &self.elements, &self.shape,
            &mut new_elements, &new_shape,
            &mut vec![0; self.shape.len()], 0,
            &axes
        );

        Self::new(new_elements, new_shape)
    }

    fn moveaxis(&self, source: Vec<isize>, destination: Vec<isize>) -> Result<Self, ArrayError> {
        source.is_unique()?;
        source.len().is_equal(&destination.len())?;
        let source = source.iter().map(|i| self.normalize_axis(*i)).collect::<Vec<usize>>();
        let destination = destination.iter().map(|i| self.normalize_axis(*i)).collect::<Vec<usize>>();
        source.is_unique()?;
        destination.is_unique()?;

        let mut order = (0 .. self.ndim()?)
            .filter(|f| !source.contains(f))
            .collect::<Vec<usize>>();

        destination.into_iter()
            .zip(source.into_iter())
            .sorted()
            .for_each(|(d, s)| order.insert(d.min(order.len()), s));

        let order = order.iter().map(|i| *i as isize).collect();
        self.transpose(Some(order))
    }

    fn rollaxis(&self, axis: isize, start: Option<isize>) -> Result<Self, ArrayError> {
        let axis = self.normalize_axis(axis);
        let start = if let Some(ax) = start { self.normalize_axis(ax) } else { 0 };

        let mut new_axes = (0 .. self.ndim()?).collect::<Vec<usize>>();
        let axis_to_move = new_axes.remove(axis);
        new_axes.insert(start, axis_to_move);

        self.transpose(Some(new_axes.iter().map(|&i| i as isize).collect()))
    }

    fn swapaxes(&self, axis_1: isize, axis_2: isize) -> Result<Self, ArrayError> {
        let axis_1 = self.normalize_axis(axis_1);
        let axis_2 = self.normalize_axis(axis_2);

        let new_axes = (0 .. self.ndim()?)
            .collect::<Vec<usize>>()
            .swap_ext(axis_1, axis_2);

        self.transpose(Some(new_axes.iter().map(|&i| i as isize).collect()))
    }

    fn expand_dims(&self, axes: Vec<isize>) -> Result<Self, ArrayError> {
        let axes = axes.iter()
            .map(|&i| self.normalize_axis_dim(i, axes.len()))
            .sorted()
            .collect::<Vec<usize>>();
        let mut new_shape = self.get_shape()?;

        for item in axes { new_shape.insert(item, 1) }
        self.reshape(&new_shape)
    }

    fn squeeze(&self, axes: Option<Vec<isize>>) -> Result<Self, ArrayError> {
        if let Some(axes) = axes {
            let axes = axes.iter()
                .map(|&i| self.normalize_axis(i))
                .sorted()
                .rev()
                .collect::<Vec<usize>>();
            let mut new_shape = self.get_shape()?;

            if axes.iter().any(|a| new_shape[*a] != 1) {
                Err(ArrayError::SqueezeShapeOfAxisMustBeOne)
            } else {
                for item in axes { new_shape.remove(item); }
                self.reshape(&new_shape)
            }
        }
        else {
            self.reshape(&self.get_shape()?.into_iter().filter(|&i| i != 1).collect::<Vec<usize>>())
        }
    }
}

impl <T: ArrayElement> ArrayAxis<T> for Result<Array<T>, ArrayError> {

    fn apply_along_axis<F>(&self, axis: usize, f: F) -> Result<Array<T>, ArrayError>
        where F: FnMut(&Array<T>) -> Result<Array<T>, ArrayError> {
        self.clone()?.apply_along_axis(axis, f)
    }

    fn transpose(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        self.clone()?.transpose(axes)
    }

    fn moveaxis(&self, source: Vec<isize>, destination: Vec<isize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.moveaxis(source, destination)
    }

    fn rollaxis(&self, axis: isize, start: Option<isize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.rollaxis(axis, start)
    }

    fn swapaxes(&self, axis: isize, start: isize) -> Result<Array<T>, ArrayError> {
        self.clone()?.swapaxes(axis, start)
    }

    fn expand_dims(&self, axes: Vec<isize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.expand_dims(axes)
    }

    fn squeeze(&self, axes: Option<Vec<isize>>) -> Result<Array<T>, ArrayError> {
        self.clone()?.squeeze(axes)
    }
}
