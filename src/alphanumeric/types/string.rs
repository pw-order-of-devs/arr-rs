use itertools::Itertools;

use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
    extensions::vec_ext::VecReverse,
};

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

    fn _append(&self, other: Self) -> Self {
        let mut result = self.to_string();
        result.push_str(other.as_str());
        result
    }

    fn _multiply(&self, n: usize) -> Self {
        self.repeat(n)
    }

    fn _capitalize(&self) -> Self {
        let mut chars: Vec<char> = self.chars().collect();
        chars[0] = chars[0].to_uppercase().next().unwrap();
        chars.into_iter().collect()
    }

    fn _lower(&self) -> Self {
        self.to_lowercase()
    }

    fn _upper(&self) -> Self {
        self.to_uppercase()
    }

    fn _swapcase(&self) -> Self {
        self.chars().map(|c| {
            if c.is_lowercase() { c.to_uppercase().to_string() }
            else if c.is_uppercase() { c.to_lowercase().to_string() }
            else { c.to_string() }
        }).collect()
    }

    fn _center(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            let char = fill_char.to_string();
            let diff = (width - self.len()) as f64 / 2.;
            format!("{}{}{}", char.repeat(diff.ceil() as usize), self, char.repeat(diff.floor() as usize))
        }
    }

    fn _join(&self, sep: Self) -> Self {
        self.chars().join(&sep)
    }

    fn _partition(&self, sep: Self) -> Tuple3<Self> {
        if let Some(index) = self.find(&sep) {
            let (before, rest) = self.split_at(index);
            let (_, after) = rest.split_at(sep.len());
            Tuple3(before.to_string(), sep, after.to_string())
        } else {
            Tuple3(self.clone(), "".to_string(), "".to_string())
        }
    }

    fn _rpartition(&self, sep: Self) -> Tuple3<Self> {
        if let Some(index) = self.rfind(&sep) {
            let (before, rest) = self.split_at(index);
            let (_, after) = rest.split_at(sep.len());
            Tuple3(before.to_string(), sep, after.to_string())
        } else {
            Tuple3(self.clone(), "".to_string(), "".to_string())
        }
    }

    fn _split(&self, sep: Self, max_split: Option<usize>) -> List<Self> {
        let result: Vec<&str> = if let Some(split) = max_split {
            self.splitn(split, &sep).collect()
        } else {
            str::split(self, &sep).collect()
        };
        List(result.into_iter().map(|s| s.to_string()).collect())
    }

    fn _rsplit(&self, sep: Self, max_split: Option<usize>) -> List<Self> {
        List(self.chars().rev().collect::<String>()._split(sep, max_split).0.reverse_ext())
    }

    fn _splitlines(&self, keep_ends: bool) -> List<Self> {
        let mut text = self.clone();
        let mut lines: Vec<String> = Vec::new();
        let mut i = 0;

        loop {
            if i >= text.len() { break }
            if text.chars().nth(i).unwrap() == '\n' || text.chars().nth(i).unwrap() == '\r' {
                if i + 1 < text.len() && text.chars().nth(i).unwrap() == '\r' && text.chars().nth(i + 1).unwrap() == '\n' {
                    if keep_ends {
                        lines.push(text.drain(0 ..= i + 1).collect());
                    } else {
                        lines.push(text.drain(0 .. i).collect());
                        text.drain(0 .. 2);
                    }
                } else if keep_ends {
                    lines.push(text.drain(0 ..= i).collect());
                } else {
                    lines.push(text.drain(0 .. i).collect());
                    text.drain(0 .. 1);
                }
                i = 0;
            } else {
                i += 1;
            }
        }

        if !text.is_empty() {
            lines.push(text.to_string());
        }

        List(lines)
    }

    fn _replace(&self, old: Self, new: Self, count: Option<usize>) -> Self {
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

    fn _strip(&self, chars: Self) -> Self {
        self._lstrip(chars.clone())._rstrip(chars)
    }

    fn _ljust(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            format!("{}{}", self, fill_char.to_string().repeat(width - self.len()))
        }
    }

    fn _lstrip(&self, chars: Self) -> Self {
        self.chars().rev().collect::<String>()._rstrip(chars).chars().rev().collect()
    }

    fn _rjust(&self, width: usize, fill_char: char) -> Self {
        if width <= self.len() {
            self.as_str()[..width].to_string()
        } else {
            format!("{}{}", fill_char.to_string().repeat(width - self.len()), self)
        }
    }

    fn _rstrip(&self, chars: Self) -> Self {
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

    fn _equal(&self, other: Self) -> bool {
        self._rstrip(" ".to_string()) == other._rstrip(" ".to_string())
    }

    fn _not_equal(&self, other: Self) -> bool {
        !self._equal(other)
    }

    fn _greater_equal(&self, other: Self) -> bool {
        self._rstrip(" ".to_string()).ge(&other._rstrip(" ".to_string()))
    }

    fn _less_equal(&self, other: Self) -> bool {
        self._rstrip(" ".to_string()).le(&other._rstrip(" ".to_string()))
    }

    fn _greater(&self, other: Self) -> bool {
        self._rstrip(" ".to_string()).gt(&other._rstrip(" ".to_string()))
    }

    fn _less(&self, other: Self) -> bool {
        self._rstrip(" ".to_string()).lt(&other._rstrip(" ".to_string()))
    }

    fn _count(&self, sub: &str) -> usize {
        self.match_indices(sub).collect::<Vec<(usize, &str)>>().len()
    }
}
