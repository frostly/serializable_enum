# Serializable Enum
[![Travis Build Status](https://img.shields.io/travis/frostly/serializable_enum.svg)](https://travis-ci.org/frostly/serializable_enum)
[![Documentation](https://img.shields.io/badge/docs-latest-C9893D.svg)](https://open.frostly.com/serializable_enum)
[![Coverage Status](https://img.shields.io/coveralls/frostly/serializable_enum.svg)](https://coveralls.io/github/frostly/serializable_enum?branch=master)
[![crates.io](https://img.shields.io/crates/v/serializable_enum.svg)](https://crates.io/crates/serializable_enum)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE-APACHE)

# Overview

Provides two macros to facilitate easier serialization / deserialization of enums with variants
having no data. The default serialization for serde when serializing enums with no data is of the
form: `{"Variant": []}`. While this works for most use cases, you may want the enum to be
serialized as `"variant"` instead. The two macros in this crate help make this
serialization/deserialization easier.

These macros are designed to be used with [serde](https://github.com/serde-rs/serde) only.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serializable_enum = "0.1.0"
```

And to your crate:

```rust,ignore
#[macro_use] extern crate serializable_enum;
```

# Example

Consider this struct:

```rust,ignore
#[derive(Serialize, Deserialize)]
struct Post {
    title: String,
    content: String,
    content_format: ContentFormat,
}

pub enum ContentFormat {
    Html,
    Markdown,
}
```

Assume an instance of `Post`:

```rust,ignore
let p = Post {
    title: String::from("I <3 serde"),
    content: String::from("awesome content"),
    content_format: ContentFormat::Markdown,
};
```

Upon serializing `Post` you want the following output (`json`):

```json
{
  "title": "I <3 serde",
  "content": "awesome content",
  "content_format": "markdown",
}
```

Using the macros in this crate, we can achieve this through the following (assuming
implementation of `Post` above):

```rust
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serializable_enum;

#[derive(Debug)]
pub enum Error {
    Parse(String)
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
```

`serializable_enum` sets up the serde serialization and deserialization using the visitor type
provided, in this case `ContentFormatVisitor`.

`impl_as_ref_from_str` provides implementations
for `AsRef` and `FromStr` traits for the enum using the mappings provided, which are used for
serialization and deserialization. `Error::Parse` is a variant of an `Error` enum defined in your
crate with `String` data. This variant is used as the `Err` type for
[FromStr](http://doc.rust-lang.org/nightly/std/str/trait.FromStr.html).

**Note**: the `serializable_enum` macro invocation **requires**:

1. Doc-comments for each variant.
2. The enum be marked `pub`. If desired, an additional macro can be made to provide the
non-pub version. PRs welcome!


For more details, head over to the [documentation](https://open.frostly.com/serializable_enum).

# License

This library is distributed under similar terms to Rust: dual licensed under the MIT license and the Apache license (version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT), and [COPYRIGHT](COPYRIGHT) for
details.
