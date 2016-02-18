#![deny(missing_debug_implementations,
        missing_docs,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_variables)]

#![cfg_attr(feature = "nightly-testing", allow(unstable_features))]
#![cfg_attr(feature = "nightly-testing", feature(plugin))]
#![cfg_attr(feature = "nightly-testing", plugin(clippy))]
#![cfg_attr(feature = "nightly-testing", deny(clippy))]

//! Macro example

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serializable_enum;

/// Error
#[derive(Debug)]
pub enum Error {
    /// Parse
    Parse(String),
}

// You will need display implemented for Error (you should already have this).
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

serializable_enum! {
    /// Supported content formats
    pub enum ContentFormat {
        /// Markdown
        Markdown,
        /// HTML
        Html,
    }
    ContentFormatVisitor
}

impl_as_ref_from_str! {
    ContentFormat {
        Markdown => "markdown",
        Html => "html",
    }
    Error::Parse
}

fn main() {
    let md = ContentFormat::Markdown;

    assert_eq!(serde_json::to_string(&md).unwrap(), "\"markdown\"");
}
