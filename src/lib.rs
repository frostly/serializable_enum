//! Macros for serializing / deserializing enums containing no data variants using serde.

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
/// # } }
#[macro_export]
macro_rules! serializable_enum {
    ($(#[$enum_comment:meta])* pub enum $name:ident {
        $($(#[$enum_variant_comment:meta])+ $variant:ident,)+
    }
    $visitor:ident) => (
        $(#[$enum_comment])*
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $($(#[$enum_variant_comment])+ $variant,)+
        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: &mut S) -> ::std::result::Result<(), S::Error> where S: ::serde::Serializer {
                self.as_ref().serialize(serializer)
            }
        }

        struct $visitor;
        impl ::serde::de::Visitor for $visitor {
            type Value = $name;

            fn visit_str<E>(&mut self, s: &str) -> ::std::result::Result<Self::Value, E>
            where E: ::serde::de::Error,
            {
                match s.trim().parse::<$name>() {
                    Ok(t) => Ok(t),
                    Err(e) => Err(::serde::de::Error::unknown_field(&e.to_string()[..])),
                }
            }
        }

        impl ::serde::Deserialize for $name {
            fn deserialize<D>(deserializer: &mut D) -> ::std::result::Result<$name, D::Error>
                    where D: ::serde::Deserializer,
                {
                    deserializer.visit_str($visitor)
                }
        }
    )
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
