# assert_matches_regex

Provides a macro, `assert_matches_regex`, which tests whether a string
matches a given regex, causing a panic if it does not match.

[![CI](https://github.com/zertosh/assert_matches_regex/workflows/CI/badge.svg)](https://github.com/zertosh/assert_matches_regex/actions)
[![Latest version](https://img.shields.io/crates/v/assert_matches_regex.svg)](https://crates.io/crates/assert_matches_regex)
[![Documentation](https://docs.rs/assert_matches_regex/badge.svg)](https://docs.rs/assert_matches_regex)
![License](https://img.shields.io/crates/l/assert_matches_regex.svg)


```toml
[dev-dependencies]
assert_matches_regex = "0.1"
```

## Example

```rust
use assert_matches_regex::assert_matches_regex;

assert_matches_regex!("Hello!", r"(?i)hello");

let data = "deadc0de";
assert_matches_regex!(data, "^[a-f0-9]$", "expected `{data}` to be a hex string");
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
