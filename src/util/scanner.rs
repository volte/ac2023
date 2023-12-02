use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use regex::{Captures, Regex};

static REGEX_CACHE: Lazy<Mutex<HashMap<&'static str, Regex>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Helper for scanning strings for tokens
#[derive(Clone, Debug)]
pub struct Scanner<'s> {
    pub string: &'s str,
    pub ignore_whitespace: bool,
}

impl Scanner<'_> {
    pub fn new(string: &str, ignore_whitespace: bool) -> Scanner {
        Scanner {
            string,
            ignore_whitespace,
        }
    }

    fn maybe_skip_whitespace(&mut self) {
        if self.ignore_whitespace {
            self.skip_whitespace();
        }
    }

    /// Scan for a constant string
    pub fn try_scan_string(&mut self, s: &str) -> Option<&str> {
        self.maybe_skip_whitespace();

        if self.string.is_empty() {
            return None;
        }
        if self.string.starts_with(&s) {
            let string = self.string;
            self.string = &self.string[s.len()..];
            Some(string)
        } else {
            None
        }
    }

    /// Scan for a regular expression match
    pub fn try_scan_regex_captures(&mut self, regex: &'static str) -> Option<Captures> {
        self.maybe_skip_whitespace();

        if self.string.is_empty() {
            return None;
        }

        let mut cache = REGEX_CACHE.lock().unwrap();
        let regex = cache
            .entry(regex)
            .or_insert_with(|| Regex::new(regex).unwrap());

        if let Some(captures) = regex.captures(self.string) {
            self.string = &self.string[captures[0].len()..];
            Some(captures)
        } else {
            None
        }
    }

    /// Scan for a regular expression match and parse it into a value that implements [`std::str::FromStr`]
    pub fn try_scan_regex_parsed<T>(&mut self, regex: &'static str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        self.maybe_skip_whitespace();

        match self.try_scan_regex_captures(regex) {
            Some(captures) => captures[0].parse::<T>().ok(),
            None => None,
        }
    }

    /// Scan for a regular expression match and return the matched string slice
    pub fn try_scan_regex(&mut self, regex: &'static str) -> Option<&str> {
        self.maybe_skip_whitespace();

        match self.try_scan_regex_captures(regex) {
            Some(captures) => Some(captures.get(0).unwrap().as_str()),
            None => None,
        }
    }

    /// Scan for an integer
    pub fn try_scan_int(&mut self) -> Option<i32> {
        self.try_scan_regex_parsed(r"^-?\d+")
    }

    /// Scan for a floating-point number
    pub fn try_scan_float(&mut self) -> Option<f32> {
        self.try_scan_regex_parsed(r"^[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)?")
    }

    /// Skip over leading whitespace
    pub fn skip_whitespace(&mut self) {
        self.string = self.string.trim_start();
    }
}
