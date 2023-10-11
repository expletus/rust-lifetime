//#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

// Struct fields need Lifetime param as well if they're references
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    /// Split haystack on delimiter
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

// Iterator for StrSplit
// Watch for revision https://youtu.be/rAl-9HwD858?t=1619
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim) + self.delimiter.len()..];
            Some(until_delimiter)
        } else if self.remainder.is_empty() {
            None
        } else {
            let rest = self.remainder;
            self.remainder = &"";
            return Some(rest);
        }
    }
}
