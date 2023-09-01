use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};

/// ArrayTrait - Array Sort functions
pub trait ArraySort<T: ArrayElement> where Self: Sized + Clone {

    /// Sort an array
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to sort. if None, array is flattened
    /// * `kind` - {‘quicksort’, ‘mergesort’, ‘heapsort’, ‘stable’}, optional. defaults to `quicksort`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(i32, [[1, 4], [3, 1]]);
    /// assert_eq!(array!(i32, [1, 1, 3, 4]), arr.sort(None, None::<&str>));
    ///
    /// let arr = array!(i32, [[1, 4], [3, 1]]);
    /// assert_eq!(array!(i32, [[1, 4], [1, 3]]), arr.sort(Some(-1), None::<&str>));
    /// ```
    fn sort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<T>, ArrayError>;

    /// Returns the indices that would sort an array
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to sort. if None, array is flattened
    /// * `kind` - {‘quicksort’, ‘mergesort’, ‘heapsort’, ‘stable’}, optional. defaults to `quicksort`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(i32, [[1, 4], [3, 1]]);
    /// assert_eq!(array!(usize, [0, 3, 2, 1]), arr.argsort(None, None::<&str>));
    ///
    /// let arr = array!(i32, [[1, 4], [3, 1]]);
    /// assert_eq!(array!(usize, [[0, 1], [1, 0]]), arr.argsort(Some(-1), None::<&str>));
    /// ```
    fn argsort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<usize>, ArrayError>;
}

impl <T: ArrayElement> ArraySort<T> for Array<T> {

    fn sort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<T>, ArrayError> {
        let kind = match kind {
            Some(k) => k.parse()?,
            None => SortKind::Quicksort,
        };
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.sort(None, Some(kind)))
        } else {
            let elements = self.get_elements()?;
            match kind {
                SortKind::Mergesort => elements.merge_sort(),
                SortKind::Quicksort => elements.quick_sort(),
                SortKind::Heapsort => elements.heap_sort(),
                SortKind::Stable => elements.tim_sort(),
            }.to_array()
        }
    }

    fn argsort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<usize>, ArrayError> {
        let kind = match kind {
            Some(k) => k.parse()?,
            None => SortKind::Quicksort,
        };
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.argsort(None, Some(kind)))
        } else {
            let elements = self.get_elements()?;
            let mut sorted = match kind {
                SortKind::Mergesort => elements.merge_sort(),
                SortKind::Quicksort => elements.quick_sort(),
                SortKind::Heapsort => elements.heap_sort(),
                SortKind::Stable => elements.tim_sort(),
            }.to_array()?.get_elements()?
                .into_iter().enumerate()
                .collect::<Vec<(usize, T)>>();

            self.get_elements()?.into_iter()
                .map(|item| {
                    let item = sorted.clone().into_iter().find(|(_, a)| a == &item).unwrap();
                    let index = sorted.clone().into_iter().position(|aa| aa == item).unwrap();
                    sorted.remove(index);
                    item.0
                })
                .collect::<Vec<usize>>()
                .to_array()
        }
    }
}

impl <T: ArrayElement> ArraySort<T> for Result<Array<T>, ArrayError> {

    fn sort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<T>, ArrayError> {
        self.clone()?.sort(axis, kind)
    }

    fn argsort(&self, axis: Option<isize>, kind: Option<impl SortKindType>) -> Result<Array<usize>, ArrayError> {
        self.clone()?.argsort(axis, kind)
    }
}
