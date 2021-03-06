//! Procedural macros for formatting and parsing fragmented strings.
//!
//! This is the main crate which reexports macros from implementation crates.
//!
//! # Formatting
//!
//! ```
//! use fragstrings::frag_format;
//! let foo = "foo";
//! let result = frag_format!("%s%s%d", foo, "bar", 42);
//! assert_eq!(result, "%s%s%d__foo__bar__42");
//! ```
//!
//! # Parsing
//!
//! ```
//! use fragstrings::frag_parse;
//! let (foo, bar, baz) = frag_parse!("%s%s%d", "%s%s%d__foo__bar__42").unwrap();
//! assert_eq!(foo, "foo");
//! assert_eq!(bar, "bar");
//! assert_eq!(baz, 42);
//! ```
//!
//! It is also allowed to ignore trailing extra fragments for the sake of extensibility,
//! using format specifier with '*' in the end:
//! ```
//! use fragstrings::frag_parse;
//! let (foo, bar) = frag_parse!("%s%s*", "%s%s%s__foo__bar__baz").unwrap();
//! assert_eq!(foo, "foo");
//! assert_eq!(bar, "bar");
//! // Fragment "baz" is silently ignored here and does not raise any errors.
//! ```

#[cfg(feature = "format")]
pub use format_procmacro::frag_format;

#[cfg(feature = "parse")]
pub use parse_procmacro::frag_parse;
