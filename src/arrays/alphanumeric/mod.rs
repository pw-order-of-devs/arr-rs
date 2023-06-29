use crate::arrays::Array;
use crate::prelude::ArrayManipulate;
use crate::traits::{
    errors::ArrayError,
    alphanumeric::ArrayString,
    create::ArrayCreate,
    manipulate::broadcast::ArrayBroadcast,
    meta::ArrayMeta,
    types::{
        alphanumeric::Alphanumeric,
        collection::List,
        tuple::tuple3::Tuple3,
    },
};

impl <N: Alphanumeric> ArrayString<N> for Array<N> {

    fn add(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.append(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn multiply(&self, counts: &Array<usize>) -> Result<Array<N>, ArrayError> {
        let arr = self.broadcast_to(counts.get_shape()?)?;
        let counts = counts.broadcast_to(arr.get_shape()?)?;
        let elements = arr.clone().into_iter().zip(counts)
            .map(|tuple| tuple.0.multiply(tuple.1))
            .collect();
        Array::new(elements, arr.get_shape()?)
    }

    fn capitalize(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s.capitalize())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn lower(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s.lower())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn upper(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s.upper())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn swapcase(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.clone().into_iter()
            .map(|s| s.swapcase())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn center(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(' ');
        let elements = self.clone().into_iter()
            .map(|s| s.center(width, fill_char))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn join(&self, sep: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.join(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.partition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError> {
        let broadcasted = self.broadcast(sep)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.rpartition(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&sep)?;
        let max_split =
            if let Some(counts) = max_split { Some(counts.broadcast_to(broadcasted.get_shape()?)?) } else { None };
        let elements = broadcasted.clone().into_iter().enumerate()
            .map(|(idx, tuple)| tuple.0.split(tuple.1, max_split.clone().map(|s| s[idx])))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rsplit(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        let sep = sep.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&sep)?;
        let max_split =
            if let Some(counts) = max_split { Some(counts.broadcast_to(broadcasted.get_shape()?)?) } else { None };
        let elements = broadcasted.clone().into_iter().enumerate()
            .map(|(idx, tuple)| tuple.0.rsplit(tuple.1, max_split.clone().map(|s| s[idx])))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn splitlines(&self, keep_ends: Option<Array<bool>>) -> Result<Array<List<N>>, ArrayError> {
        let keep_ends = keep_ends.unwrap_or(Array::single(false)?);
        let tmp_keep_ends = Array::single(N::from_str(" ")).broadcast_to(keep_ends.get_shape()?)?;
        let tmp_array = self.broadcast(&tmp_keep_ends)?;

        let array = tmp_array.clone().into_iter()
            .map(|t| t.0).collect::<Array<N>>()
            .reshape(tmp_array.get_shape()?)?;
        let keep_ends = keep_ends.broadcast_to(array.get_shape()?)?;
        let elements = array.clone().into_iter().enumerate()
            .map(|(idx, elem)| elem.splitlines(keep_ends[idx]))
            .collect();
        Array::new(elements, array.get_shape()?)
    }

    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Result<Array<N>, ArrayError> {
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), old.clone(), new.clone()])?;
        let tupled = (0 .. broadcasted[0].len()?).map(|i| {
            Tuple3(broadcasted[0][i].clone(), broadcasted[1][i].clone(), broadcasted[2][i].clone())
        }).collect::<Vec<Tuple3<N>>>();
        let elements = tupled.into_iter()
            .map(|tuple| tuple.0.replace(tuple.1, tuple.2, count))
            .collect();
        Array::new(elements, broadcasted[0].get_shape()?)
    }

    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.lstrip(chars.clone()).rstrip(chars)
    }

    fn ljust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(' ');
        let elements = self.clone().into_iter()
            .map(|s| s.ljust(width, fill_char))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        let chars = chars.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.lstrip(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn rjust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        let fill_char = fill_char.unwrap_or(' ');
        let elements = self.clone().into_iter()
            .map(|s| s.rjust(width, fill_char))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        let chars = chars.unwrap_or(Array::single(N::from_str(" "))?);
        let broadcasted = self.broadcast(&chars)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| tuple.0.rstrip(tuple.1))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }
}


impl <N: Alphanumeric> ArrayString<N> for Result<Array<N>, ArrayError> {

    fn add(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.add(other)
    }

    fn multiply(&self, counts: &Array<usize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.multiply(counts)
    }

    fn capitalize(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.capitalize()
    }

    fn lower(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.lower()
    }

    fn upper(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.upper()
    }

    fn swapcase(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.swapcase()
    }

    fn center(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        self.clone()?.center(width, fill_char)
    }

    fn join(&self, sep: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.join(sep)
    }

    fn partition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError> {
        self.clone()?.partition(sep)
    }

    fn rpartition(&self, sep: &Array<N>) -> Result<Array<Tuple3<N>>, ArrayError> {
        self.clone()?.rpartition(sep)
    }

    fn split(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        self.clone()?.split(sep, max_split)
    }

    fn rsplit(&self, sep: Option<Array<N>>, max_split: Option<Array<usize>>) -> Result<Array<List<N>>, ArrayError> {
        self.clone()?.rsplit(sep, max_split)
    }

    fn splitlines(&self, keep_ends: Option<Array<bool>>) -> Result<Array<List<N>>, ArrayError> {
        self.clone()?.splitlines(keep_ends)
    }

    fn replace(&self, old: &Array<N>, new: &Array<N>, count: Option<usize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.replace(old, new, count)
    }

    fn strip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.strip(chars)
    }

    fn ljust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        self.clone()?.ljust(width, fill_char)
    }

    fn lstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.lstrip(chars)
    }

    fn rjust(&self, width: usize, fill_char: Option<char>) -> Result<Array<N>, ArrayError> {
        self.clone()?.rjust(width, fill_char)
    }

    fn rstrip(&self, chars: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.rstrip(chars)
    }
}
