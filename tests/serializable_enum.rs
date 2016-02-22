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
    #[derive(Debug, PartialEq)]
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

serializable_enum! {
    /// Supported content formats
    #[derive(Debug, PartialEq)]
    enum PrivContentFormat {
        /// Markdown
        Markdown,
        /// HTML no comma
        Html
    }
    PrivContentFormatVisitor
}
impl_as_ref_from_str! {
    PrivContentFormat {
        Markdown => "markdown",
        Html => "html",
    }
    Error::Parse
}

// no derive
serializable_enum! {
    /// NoDeriveContentFormat
    enum NoDeriveContentFormat {
        /// Markdown
        Markdown,
        /// HTML no comma
        Html
    }
    NoDeriveContentFormatVisitor
}
impl_as_ref_from_str! {
    NoDeriveContentFormat {
        Markdown => "markdown",
        Html => "html",
    }
    Error::Parse
}

#[test]
fn test_pub_serialization() {
    let md = ContentFormat::Markdown;
    assert_eq!(serde_json::to_string(&md).unwrap(), "\"markdown\"");

    let des_md: ContentFormat = serde_json::from_str("\"markdown\"").unwrap();
    assert_eq!(md, des_md);
}

#[test]
fn test_priv_serialization() {
    let md = PrivContentFormat::Markdown;
    assert_eq!(serde_json::to_string(&md).unwrap(), "\"markdown\"");

    let des_md: PrivContentFormat = serde_json::from_str("\"markdown\"").unwrap();
    assert_eq!(md, des_md);
}
