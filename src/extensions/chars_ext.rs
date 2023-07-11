use std::str::Chars;

pub(crate) trait CharsJoin {

    fn join(&mut self, sep: &str) -> String;
}

impl CharsJoin for Chars<'_> {

    fn join(&mut self, sep: &str) -> String {
        self.fold(String::new(), |mut acc, c| {
            if !acc.is_empty() { acc.push_str(sep); }
            acc.push(c);
            acc
        })
    }
}
