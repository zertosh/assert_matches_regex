//! Provides a macro, [`assert_matches_regex!`], which tests whether a string
//! matches a given regex, causing a panic if it does not match.
//!
//! [`assert_matches_regex!`]: macro.assert_matches_regex.html

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

/// A re-export of [`regex::escape`] for convenience.
///
/// [`regex::escape`]: https://docs.rs/regex/*/regex/fn.escape.html
pub use regex::escape;

#[doc(hidden)]
pub mod __private {
    pub use regex;
}

/// Asserts that a string matches a regex using [`regex::Regex`].
///
/// [`regex::Regex`]: https://docs.rs/regex/*/regex/struct.Regex.html
///
/// # Examples
///
/// ```
/// # use assert_matches_regex::assert_matches_regex;
/// assert_matches_regex!("Hello!", r"(?i)hello");
/// ```
///
/// The haystack can be a `String` or `&str`.
///
/// ```
/// # use assert_matches_regex::assert_matches_regex;
/// assert_matches_regex!(format!("{:?}", vec![1, 2, 3]), r"\[1.*3\]");
///
/// let duration = std::time::Duration::from_secs(5);
/// assert_matches_regex!(duration.as_millis().to_string(), "^50{3}$");
/// ```
///
/// An optional message in the form of a format string can be passed last.
///
/// ```rust,should_panic
/// # use assert_matches_regex::assert_matches_regex;
/// let data = "foo bar";
/// assert_matches_regex!(data, "^[a-f0-9]$", "expected `{data}` to be a hex string");
/// ```
#[macro_export]
macro_rules! assert_matches_regex {
    ($haystack:expr, $re:expr $(,)?) => {{
        let haystack = $haystack;
        let re = $crate::__private::regex::Regex::new(&$re).expect("a valid regex");
        if !re.is_match(&haystack) {
            ::std::panic!(
                "assertion failed: `{haystack:?}` does not match `{}`",
                re.as_str(),
            );
        }
    }};
    ($haystack:expr, $re:expr, $($arg:tt)+) => {{
        let haystack = $haystack;
        let re = $crate::__private::regex::Regex::new(&$re).expect("a valid regex");
        if !re.is_match(&haystack) {
            ::std::panic!(
                "assertion failed: `{haystack:?}` does not match `{}`: {}",
                re.as_str(),
                ::std::format_args!($($arg)*),
            );
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::assert_matches_regex;

    macro_rules! assert_panic {
        ($expr:expr, $msg:expr) => {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $expr)) {
                Ok(_) => panic!("expression did not panic; `{}`", stringify!($expr)),
                Err(err) => {
                    let expected = $msg.to_string();
                    if let Some(actual) = err.downcast_ref::<String>() {
                        assert_eq!(*actual, expected, "panic message does not match");
                    } else {
                        panic!("panic message not found");
                    }
                }
            }
        };
    }

    #[test]
    fn trailing_comma() {
        assert_matches_regex!("abc", r"\w");
        assert_matches_regex!("abc", r"\w",);
    }

    #[test]
    fn string_types() {
        assert_matches_regex!("abc", String::from(r"\w"));
        assert_matches_regex!("abc", &String::from(r"\w"));
        assert_matches_regex!(String::from("abc"), r"\w");
        assert_matches_regex!(&String::from("abc"), r"\w");

        assert_matches_regex!(std::borrow::Cow::Borrowed("abc"), r"\w");
        assert_matches_regex!(String::from_utf8_lossy(b"abc"), r"\w");
        assert_matches_regex!(String::from_utf8_lossy(b"abc"), r"\w");
    }

    #[test]
    fn mismatch_no_message() {
        assert_panic!(
            assert_matches_regex!("abc", r"\d"),
            r#"assertion failed: `"abc"` does not match `\d`"#
        );
    }

    #[test]
    fn mismatch_message_no_format() {
        assert_panic!(
            assert_matches_regex!("abc", r"\d", "XXX"),
            r#"assertion failed: `"abc"` does not match `\d`: XXX"#
        );
    }

    #[test]
    fn mismatch_message_format() {
        assert_panic!(
            assert_matches_regex!("abc", r"\d", "value={}", "XXX"),
            r#"assertion failed: `"abc"` does not match `\d`: value=XXX"#
        );
    }

    #[test]
    #[should_panic(expected = "regex parse error")]
    fn bad_regex() {
        assert_matches_regex!("abc", r"[a-z");
    }
}
