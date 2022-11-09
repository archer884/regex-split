//! The trait `RegexSplit` adds a pair of split options to `Regex`: `split_inclusive` and
//! `split_inclusive_left`. `split_inclusive` works similarly to the method of the same name in
//! std, where `split_inclusive_left` includes the delimiter at the front of each substring instead
//! of at the back.
//! 
//! ## `split_inclusive`
//! 
//! It's possible to roll your own `lines()` function. Why? I don't know, but you can.
//! 
//! ```rust
//! # use regex::Regex;
//! # use crate::regex_split::RegexSplit;
//! # fn main() {
//! let re = Regex::new("\r?\n").unwrap();
//! let text = "This is just\na set of lines\r\nwith different newlines.";
//! let v: Vec<&str> = re.split_inclusive(text).collect();
//! assert_eq!(v, [
//!     "This is just\n",
//!     "a set of lines\r\n",
//!     "with different newlines.",
//! ]);
//! # }
//! ```
//! 
//! ## `split_inclusive_left`
//! 
//! This is useful if your delimiter includes some context that is associated with the substring
//! to the *right.* Is that useful? No, not generally--but there's really no substitute if you
//! need it.
//! 
//! ```rust
//! # use regex::Regex;
//! # use crate::regex_split::RegexSplit;
//! # fn main() {
//! let re = Regex::new("(?m)^-").unwrap();
//! let text = "List of fruits:\n-apple\n-pear\n-banana";
//! let v: Vec<&str> = re.split_inclusive_left(text).collect();
//! assert_eq!(v, [
//!     "List of fruits:\n",
//!     "-apple\n",
//!     "-pear\n",
//!     "-banana",
//! ]);
//! # }
//! ```
//!  
//! Use `regex_split::bytes::RegexSplit` for `regex::bytes::Regex`.

pub mod bytes;

use std::iter::FusedIterator;

use regex::{Matches, Regex};

pub trait RegexSplit {
    fn split_inclusive<'r, 't>(&'r self, text: &'t str) -> SplitInclusive<'r, 't>;
    fn split_inclusive_left<'r, 't>(&'r self, text: &'t str) -> SplitInclusiveLeft<'r, 't>;
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
    text: &'t str,
}

impl<'r, 't> Iterator for SplitInclusive<'r, 't> {
    type Item = &'t str;

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
    text: &'t str,
}

impl<'r, 't> Iterator for SplitInclusiveLeft<'r, 't> {
    type Item = &'t str;

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
    /// # use regex::Regex;
    /// # use crate::regex_split::RegexSplit;
    /// # fn main() {
    /// let re = Regex::new(r"\r?\n").unwrap();
    /// let text = "Mary had a little lamb\nlittle lamb\r\nlittle lamb.";
    /// let v: Vec<&str> = re.split_inclusive(text).collect();
    /// assert_eq!(v, [
    ///     "Mary had a little lamb\n",
    ///     "little lamb\r\n",
    ///     "little lamb.",
    /// ]);
    /// # }
    /// ```
    fn split_inclusive<'r, 't>(&'r self, text: &'t str) -> SplitInclusive<'r, 't> {
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
    /// # use regex::Regex;
    /// # use crate::regex_split::RegexSplit;
    /// # fn main() {
    /// let re = Regex::new(r"\r?\n").unwrap();
    /// let text = "Mary had a little lamb\nlittle lamb\r\nlittle lamb.";
    /// let v: Vec<&str> = re.split_inclusive_left(text).collect();
    /// assert_eq!(v, [
    ///     "Mary had a little lamb",
    ///     "\nlittle lamb",
    ///     "\r\nlittle lamb.",
    /// ]);
    /// # }
    /// ```
    fn split_inclusive_left<'r, 't>(&'r self, text: &'t str) -> SplitInclusiveLeft<'r, 't> {
        SplitInclusiveLeft {
            finder: self.find_iter(text),
            last: 0,
            text,
        }
    }
}
