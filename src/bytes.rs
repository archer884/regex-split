use std::iter::FusedIterator;

use regex::bytes::{Matches, Regex};

pub trait RegexSplit {
    fn split_inclusive<'r, 't>(&'r self, text: &'t [u8]) -> SplitInclusive<'r, 't>;
    fn split_inclusive_left<'r, 't>(&'r self, text: &'t [u8]) -> SplitInclusiveLeft<'r, 't>;
}

/// Yields all substrings delimited by a regular expression match inclusive of
/// the match.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the byte string being split.
#[derive(Debug)]
pub struct SplitInclusive<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,

    // The internals of finder are private, meaning we need to keep a reference
    // to the text for ourselves. This differs from the previous
    // implementation.
    text: &'t [u8],
}

impl<'r, 't> Iterator for SplitInclusive<'r, 't> {
    type Item = &'t [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.finder.next() {
            None => {
                if self.last > self.text.len() {
                    None
                } else {
                    let s = &self.text[self.last..];
                    self.last = self.text.len() + 1; // Next call will return None
                    Some(s)
                }
            }
            Some(m) => {
                let matched = &self.text[self.last..m.end()];
                self.last = m.end();
                Some(matched)
            }
        }
    }
}

impl<'r, 't> FusedIterator for SplitInclusive<'r, 't> {}

/// Yields all substrings delimited by a regular expression match inclusive of
/// the match.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the byte string being split.
#[derive(Debug)]
pub struct SplitInclusiveLeft<'r, 't> {
    finder: Matches<'r, 't>,
    last: usize,

    // The internals of finder are private, meaning we need to keep a reference
    // to the text for ourselves. This differs from the previous
    // implementation.
    text: &'t [u8],
}

impl<'r, 't> Iterator for SplitInclusiveLeft<'r, 't> {
    type Item = &'t [u8];

    fn next(&mut self) -> Option<Self::Item> {
        match self.finder.next() {
            None => {
                if self.last > self.text.len() {
                    None
                } else {
                    let s = &self.text[self.last..];
                    self.last = self.text.len() + 1; // Next call will return None
                    Some(s)
                }
            }
            Some(m) => {
                let matched = &self.text[self.last..m.start()];
                self.last = m.start();
                Some(matched)
            }
        }
    }
}

impl<'r, 't> FusedIterator for SplitInclusiveLeft<'r, 't> {}

impl RegexSplit for Regex {
    /// Returns an iterator of substrings of `text` separated by a match of the
    /// regular expression. Differs from the iterator produced by split in that
    /// split_inclusive leaves the matched part as the terminator of the
    /// substring.
    ///
    /// This method will *not* copy the text given.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # use crate::regex_split::bytes::RegexSplit;
    /// # fn main() {
    /// let re = Regex::new(r"\r?\n").unwrap();
    /// let text = b"Mary had a little lamb\nlittle lamb\r\nlittle lamb.";
    /// let v: Vec<&[u8]> = re.split_inclusive(text).collect();
    /// assert_eq!(v, [
    ///     &b"Mary had a little lamb\n"[..],
    ///     &b"little lamb\r\n"[..],
    ///     &b"little lamb."[..]
    /// ]);
    /// # }
    /// ```
    fn split_inclusive<'r, 't>(&'r self, text: &'t [u8]) -> SplitInclusive<'r, 't> {
        SplitInclusive {
            finder: self.find_iter(text),
            last: 0,
            text,
        }
    }

    /// Returns an iterator of substrings of `text` separated by a match of the
    /// regular expression. Differs from the iterator produced by split in that
    /// split_inclusive leaves the matched part as the terminator of the
    /// substring.
    ///
    /// This method will *not* copy the text given.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use regex::bytes::Regex;
    /// # use crate::regex_split::bytes::RegexSplit;
    /// # fn main() {
    /// let re = Regex::new(r"\r?\n").unwrap();
    /// let text = b"Mary had a little lamb\nlittle lamb\r\nlittle lamb.";
    /// let v: Vec<&[u8]> = re.split_inclusive_left(text).collect();
    /// assert_eq!(v, [
    ///     &b"Mary had a little lamb"[..],
    ///     &b"\nlittle lamb"[..],
    ///     &b"\r\nlittle lamb."[..]
    /// ]);
    /// # }
    /// ```
    fn split_inclusive_left<'r, 't>(&'r self, text: &'t [u8]) -> SplitInclusiveLeft<'r, 't> {
        SplitInclusiveLeft {
            finder: self.find_iter(text),
            last: 0,
            text,
        }
    }
}
