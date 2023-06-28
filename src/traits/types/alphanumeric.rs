use itertools::Itertools;
use crate::prelude::Tuple3;
use crate::traits::types::ArrayElement;

/// Alphanumeric trait for array
pub trait Alphanumeric: ArrayElement {

    /// parse from &str
    fn from_str(str: &str) -> Self;

    /// append string with another
    fn append(&self, other: Self) -> Self;

    /// multiply string n-times
    fn multiply(&self, n: usize) -> Self;

    /// capitalize string
    fn capitalize(&self) -> Self;

    /// lower case string
    fn lower(&self) -> Self;

    /// upper case string
    fn upper(&self) -> Self;

    /// center string elements
    fn center(&self, width: usize, fill_char: char) -> Self;

    /// join string by separator
    fn join(&self, sep: Self) -> Self;

    /// partition string by first occurrence of separator
    fn partition(&self, sep: Self) -> Tuple3<Self>;

    /// partition string by last occurrence of separator
    fn rpartition(&self, sep: Self) -> Tuple3<Self>;

    /// replace <old> string with <new> <count> times
    fn replace(&self, old: Self, new: Self, count: Option<usize>) -> Self;

    /// strips string elements
    fn strip(&self, chars: Self) -> Self;

    /// left-justifies string elements
    fn ljust(&self, width: usize, fill_char: char) -> Self;

    /// left-strips string elements
    fn lstrip(&self, chars: Self) -> Self;

    /// right-justifies string elements
    fn rjust(&self, width: usize, fill_char: char) -> Self;

    /// right-strips string elements
    fn rstrip(&self, chars: Self) -> Self;
}

impl ArrayElement for String {

    fn zero() -> Self {
        "0".to_string()
    }

    fn one() -> Self {
        "1".to_string()
    }
}

impl Alphanumeric for String {

    fn from_str(str: &str) -> Self {
        str.to_string()
    }

    fn append(&self, other: Self) -> Self {
        let mut result = self.to_string();
        result.push_str(other.as_str());
        result
    }

    fn multiply(&self, n: usize) -> Self {
        self.repeat(n)
    }

    fn capitalize(&self) -> Self {
        let mut chars: Vec<char> = self.chars().collect();
        chars[0] = chars[0].to_uppercase().next().unwrap();
        chars.into_iter().collect()
    }

    fn lower(&self) -> Self {
        self.to_lowercase()
    }

    fn upper(&self) -> Self {
        self.to_uppercase()
    }

    fn center(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            let char = fill_char.to_string();
            let diff = (width - self.len()) as f64 / 2.;
            format!("{}{}{}", char.repeat(diff.ceil() as usize), self, char.repeat(diff.floor() as usize))
        }
    }

    fn join(&self, sep: Self) -> Self {
        self.chars().join(&sep)
    }

    fn partition(&self, sep: Self) -> Tuple3<Self> {
        if let Some(index) = self.find(&sep) {
            let (before, rest) = self.split_at(index);
            let (_, after) = rest.split_at(sep.len());
            Tuple3(before.to_string(), sep, after.to_string())
        } else {
            Tuple3(self.clone(), "".to_string(), "".to_string())
        }
    }

    fn rpartition(&self, sep: Self) -> Tuple3<Self> {
        if let Some(index) = self.rfind(&sep) {
            let (before, rest) = self.split_at(index);
            let (_, after) = rest.split_at(sep.len());
            Tuple3(before.to_string(), sep, after.to_string())
        } else {
            Tuple3(self.clone(), "".to_string(), "".to_string())
        }
    }

    fn replace(&self, old: Self, new: Self, count: Option<usize>) -> Self {
        let mut replaced_count = 0;
        let mut replaced_string = self.clone();

        while let Some(index) = replaced_string.find(old.as_str()) {
            if count.is_some() && replaced_count >= count.unwrap() {
                break;
            }

            replaced_string.replace_range(index .. index + old.len(), new.as_str());
            replaced_count += 1;
        }

        replaced_string
    }

    fn strip(&self, chars: Self) -> Self {
        self.lstrip(chars.clone()).rstrip(chars)
    }

    fn ljust(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            format!("{}{}", self, fill_char.to_string().repeat(width - self.len()))
        }
    }

    fn lstrip(&self, chars: Self) -> Self {
        self.chars().rev().collect::<String>().rstrip(chars).chars().rev().collect()
    }

    fn rjust(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            format!("{}{}", fill_char.to_string().repeat(width - self.len()), self)
        }
    }

    fn rstrip(&self, chars: Self) -> Self {
        let mut result = self.clone();
        loop {
            if result.is_empty() { return result }
            if chars.contains(result.chars().last().unwrap()) {
                result.remove(result.len() - 1);
            } else {
                return result
            }
        }
    }
}
