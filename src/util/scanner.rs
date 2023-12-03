#![allow(unused)]

use std::{collections::HashMap, str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;
use regex::{Regex, RegexBuilder};

static REGEX_CACHE: Lazy<Mutex<HashMap<&'static str, Regex>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Helper for scanning strings for tokens
#[derive(Clone, Debug)]
pub struct Scanner<'s> {
    string: &'s str,
    offset: usize,
    ignore_whitespace: bool,
    debug: bool,
}

/// A token that was scanned from a string
///
/// The type parameter `T` is the type of the token's value
#[derive(Clone, Debug)]
pub struct Token<'s> {
    string: &'s str,
    start: usize,
    end: usize,
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<'s> Token<'s> {
    /// Create a new token from an input string slice and a range
    pub fn new(string: &'s str, start: usize, end: usize) -> Token<'s> {
        Token { string, start, end }
    }

    /// Return the token's value as a string slice
    pub fn as_str(&self) -> &'s str {
        &self.string[self.start..self.end]
    }

    /// Attempt to parse the token's value and return the result, or None if parsing fails
    pub fn try_parse<T: FromStr>(&self) -> Option<T> {
        self.string[self.start..self.end].parse::<T>().ok()
    }

    /// Parse the token's value and return the result, or panic if parsing fails
    pub fn parse<T: FromStr>(&self) -> T {
        self.try_parse().expect("Failed to parse token")
    }

    /// Return the token's offset relative to the start of the input
    pub fn offset(&self) -> usize {
        self.start
    }

    /// Return the length of the token
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

impl<'s> Scanner<'s> {
    /// Create a new scanner from an input string
    ///
    /// If `ignore_whitespace` is true, the scanner will skip over whitespace before scanning for tokens.
    /// Tokens themselves may still contain whitespace.
    pub fn new(string: &str, ignore_whitespace: bool) -> Scanner {
        Scanner {
            string,
            ignore_whitespace,
            offset: 0,
            debug: false,
        }
    }

    /// Enable debug mode, which will print debug information to stdout
    pub fn enable_debug(mut self) -> Scanner<'s> {
        self.debug = true;
        self
    }

    fn maybe_skip_whitespace(&mut self) {
        if self.ignore_whitespace {
            self.skip_whitespace();
        }
    }

    fn consume_token(&mut self, len: usize) -> Token<'s> {
        let offset = self.offset;
        self.offset += len;
        let result = Token::new(self.string, offset, offset + len);

        if self.debug {
            println!("Consumed token: {:?}", result.as_str());
        }

        result
    }

    /// Seek to a particular offset in the input
    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
    }

    /// Return the slice of the string that remains to be consumed
    pub fn remaining(&self) -> &'s str {
        &self.string[self.offset..]
    }

    /// Return true if there is no more input to consume
    pub fn is_finished(&self) -> bool {
        if self.ignore_whitespace {
            self.remaining().trim().is_empty()
        } else {
            self.remaining().is_empty()
        }
    }

    /// Return the current offset relative to the start of the input
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Scan for a constant string and return the matched string slice, or None if no match is found
    pub fn try_scan_string(&mut self, s: &str) -> Option<Token<'s>> {
        self.maybe_skip_whitespace();

        if self.remaining().starts_with(&s) {
            Some(self.consume_token(s.len()))
        } else {
            None
        }
    }

    /// Scan for a constant string and return the matched string slice, or panic if no match is found
    pub fn scan_string(&mut self, s: &str) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_string(s)
            .expect(format!("Expected string '{}' at {}", s, offset).as_str())
    }

    /// Scan for a regular expression match and return the matched string slice, or None if no match is found
    pub fn try_scan_regex(&mut self, regex: &'static str) -> Option<Token<'s>> {
        self.maybe_skip_whitespace();

        if self.is_finished() {
            return None;
        }

        let mut cache = REGEX_CACHE.lock().unwrap();
        let regex = cache.entry(regex).or_insert_with(|| {
            RegexBuilder::new(format!("^{}", regex).as_str())
                .build()
                .unwrap()
        });

        if let Some(m) = regex.find(self.remaining()) {
            if self.debug {
                println!("Regex match: {:?}", m);
            }
            Some(self.consume_token(m.len()))
        } else {
            None
        }
    }

    /// Scan for a regular expression match and return the matched string slice, or panic if no match is found
    pub fn scan_regex(&mut self, regex: &'static str) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_regex(regex)
            .expect(format!("Expected regex '{}' at {}", regex, offset).as_str())
    }

    /// Scan for a linebreak and return the matched string slice, or None if no match is found
    pub fn try_scan_linebreak(&mut self) -> Option<Token<'s>> {
        self.try_scan_regex("(\r\n|\r|\n)")
    }

    /// Scan for a linebreak and return the matched string slice, or panic if no match is found
    pub fn scan_linebreak(&mut self) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_linebreak()
            .expect(format!("Expected linebreak at {}", offset).as_str())
    }

    /// Scan for a signed integer and return the matched string slice, or None if no match is found
    pub fn try_scan_signed_int(&mut self) -> Option<Token<'s>> {
        self.try_scan_regex(r"-?\d+")
    }

    /// Scan for a signed integer and return the matched string slice, or panic if no match is found
    pub fn scan_signed_int(&mut self) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_signed_int()
            .expect(format!("Expected integer at {}", offset).as_str())
    }

    /// Scan for an unsigned integer and return the matched string slice, or None if no match is found
    pub fn try_scan_unsigned_int(&mut self) -> Option<Token<'s>> {
        self.try_scan_regex(r"\d+")
    }

    /// Scan for an unsigned integer and return the matched string slice, or panic if no match is found
    pub fn scan_unsigned_int(&mut self) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_unsigned_int()
            .expect(format!("Expected unsigned integer at {}", offset).as_str())
    }

    /// Scan for a floating-point number and return the matched string slice, or None if no match is found
    pub fn try_scan_float(&mut self) -> Option<Token<'s>> {
        self.try_scan_regex(r"[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?")
    }

    /// Scan for a floating-point number and return the matched string slice, or panic if no match is found
    pub fn scan_float(&mut self) -> Token<'s> {
        let offset = self.offset;
        self.try_scan_float()
            .expect(format!("Expected float at {}", offset).as_str())
    }

    /// Skip over leading whitespace
    pub fn skip_whitespace(&mut self) {
        self.offset += self
            .remaining()
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(0);
    }
}
