//! Macros for serializing / deserializing enums containing no data variants using serde.

/// Implement serde Serialize, Deserialize, and Visitor traits for the provided type and visitor
/// type.
#[macro_export]
macro_rules! serde_visitor {
    ($name:ident, $visitor:ident, $($variant:ident),+) => (
        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> where S: ::serde::Serializer {
                self.as_ref().serialize(serializer)
            }
        }

        struct $visitor;
        impl<'de> ::serde::de::Visitor<'de> for $visitor {
            type Value = $name;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("a str")
            }

            fn visit_str<E>(self, s: &str) -> ::std::result::Result<Self::Value, E>
            where E: ::serde::de::Error,
            {
                #[allow(non_upper_case_globals)]
                const VARIANTS: &'static [&'static str] = &[$(stringify!($variant)),+];

                match s.trim().parse::<$name>() {
                    Ok(t) => Ok(t),
                    Err(e) => Err(::serde::de::Error::unknown_field(&e.to_string()[..], VARIANTS)),
                }
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<$name, D::Error>
                    where D: ::serde::Deserializer<'de>,
                {
                    deserializer.deserialize_str($visitor)
                }
        }
    )
}

/// Implements deserialization and serialization for an enum of the form:
///
/// ```
/// # mod a {
/// pub enum Color {
///     Red,
///     Blue,
/// }
/// # }
/// ```
///
/// to serialize `Color::Red` to `"red"`. This overrides serde's default behavior of `{"Red":[]}`.
/// One must pass an identifier to use as the type's Visitor.
///
/// Relies on the type implementing `FromStr` to call `parse` when deserializing.
///
/// Relies on `AsRef` implementation when serializing.
///
///
/// # Example
///
/// ```
/// #[macro_use] extern crate serializable_enum;
/// extern crate serde;
/// # fn main() {
/// # mod a {
///
/// // your error type
/// #[derive(Debug)]
/// pub enum Error {
///     Parse(String),
/// }
///
/// // You will need display implemented (you should already have this).
/// impl ::std::fmt::Display for Error {
///    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
///        write!(f, "{:?}", self)
///    }
/// }
///
/// serializable_enum! {
///     /// Color
///     #[derive(Debug, PartialEq)]
///     pub enum Color {
///         /// Red
///         Red,
///         /// Blue
///         Blue,
///         /// Green
///         Green,
///     }
///     ColorVistor
/// }
///
/// impl_as_ref_from_str! {
///     Color {
///         Red => "red",
///         Blue => "blue",
///         Green => "green",
///     }
///     Error::Parse
/// }
/// // also works with non-pub enums
///
/// serializable_enum! {
///     /// ContentFormat
///     #[derive(Debug)]
///     enum ContentFormat {
///         /// Markdown
///         Markdown,
///         /// Html
///         Html,
///     }
///     ContentFormatVisitor
/// }
/// impl_as_ref_from_str! {
///     ContentFormat {
///         Markdown => "md",
///         Html => "html",
///     }
///     Error::Parse
/// }
/// # } }
#[macro_export]
macro_rules! serializable_enum {
    // pub enum
    {
        $(#[$enum_meta:meta])+
        pub enum $name:ident {
            $($(#[$enum_variant_comment:meta])+ $variant:ident),+
            $(,)*
        }
        $visitor:ident
    } => {
        $(#[$enum_meta])+
        pub enum $name {
            $($(#[$enum_variant_comment])+ $variant,)+
        }
        serde_visitor!($name, $visitor, $($variant),+);
    };
    // no pub
    {
        $(#[$enum_meta:meta])+
        enum $name:ident {
            $($(#[$enum_variant_comment:meta])+ $variant:ident),+
            $(,)*
        }
        $visitor:ident
    } => {
        $(#[$enum_meta])+
        enum $name {
            $($(#[$enum_variant_comment])+ $variant,)+
        }
        serde_visitor!($name, $visitor, $($variant),+);
    };
}

/// Generate `AsRef` and `FromStr` impls for the given type with the variant / string pairs
/// specified.
///
/// # Example
///
/// ```
/// #[macro_use] extern crate serializable_enum;
/// # fn main() { mod a {
///
/// // your error type
/// #[derive(Debug)]
/// enum Error {
///     Parse(String),
/// }
///
/// // You will need display implemented (you should already have this).
/// impl ::std::fmt::Display for Error {
///    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
///        write!(f, "{:?}", self)
///    }
/// }
///
/// enum Color {
///     /// Red
///     Red,
///     /// Blue
///     Blue,
///     /// Green
///     Green,
/// }
///
/// impl_as_ref_from_str! {
///     Color {
///         Red => "red",
///         Blue => "blue",
///         Green => "green",
///     }
///     Error::Parse
/// }
/// # } }
#[macro_export]
macro_rules! impl_as_ref_from_str {
    ($name:ident {
        $($variant:ident => $str:expr,)+
    }
    $err:ident::$err_variant:ident
    ) => (
        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                match *self {
                    $($name::$variant=> $str,)+
                }
            }
        }
        impl ::std::str::FromStr for $name {
            type Err = $err;
            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $($str => Ok($name::$variant),)+
                    _ => Err($err::$err_variant(format!("`{}` is not a known `{}` variant", s, stringify!($name)))),
                }
            }

        }
    )
}
