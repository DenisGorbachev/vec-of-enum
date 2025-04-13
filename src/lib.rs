//! A helper struct to manage a `Vec` of `enum` values. Reduces boilerplate, implements useful traits.
//!
//! ```rust
//! # use derive_more::{Constructor, From};
//! # use serde::{Deserialize, Serialize};
//! #
//! # // Define some sample validation error structs
//! # #[derive(Constructor, Serialize, Deserialize)]
//! # pub struct PasswordMinLengthError {
//! #     min_length: usize,
//! # }
//! #
//! # #[derive(Constructor, Serialize, Deserialize)]
//! # pub struct InvalidEmailError {
//! #     email: String,
//! #     reason: String,
//! # }
//! #
//! # // Define an enum that can contain any validation error
//! # #[derive(From, Serialize, Deserialize)]
//! # pub enum ValidationError {
//! #     PasswordMinLength(PasswordMinLengthError),
//! #     InvalidEmail(InvalidEmailError),
//! # }
//! #
//! # // Convenience conversion
//! # impl From<(&str, &str)> for ValidationError {
//! #     fn from((email, reason): (&str, &str)) -> Self {
//! #         Self::InvalidEmail(InvalidEmailError::new(email.into(), reason.into()))
//! #     }
//! # }
//! #
//! # // Define a typed vector wrapper for ValidationErrors
//! # vec_of_enum::define!(
//! #     #[derive(Serialize, Deserialize)]
//! #     pub struct ValidationErrors(Vec<ValidationError>);
//! # );
//! #
//! # // Define a typed vector wrapper that also automatically converts from variant types
//! # vec_of_enum::define!(
//! #     #[derive(Serialize, Deserialize)]
//! #     pub struct ValidationErrorsWithVariants(Vec<ValidationError>);
//! #     variants = [PasswordMinLengthError, InvalidEmailError];
//! # );
//! #
//! let mut errors = ValidationErrors::default();
//!
//! // ❌ Without `vec-of-enum`: too verbose
//! errors.push(ValidationError::InvalidEmail(InvalidEmailError::new("user@example.com".into(), "domain is blocked".into())));
//!
//! // ✅ With `vec-of-enum`: very concise
//! errors.push(("user@example.com", "domain is blocked"));
//! ```
//!
//! # Full example
//!
//! ```rust
//! use derive_more::{Constructor, From};
//! use serde::{Deserialize, Serialize};
//!
//! // Define some sample validation error structs
//! #[derive(Constructor, Serialize, Deserialize)]
//! pub struct PasswordMinLengthError {
//!     min_length: usize,
//! }
//!
//! #[derive(Constructor, Serialize, Deserialize)]
//! pub struct InvalidEmailError {
//!     email: String,
//!     reason: String,
//! }
//!
//! // Define an enum that can contain any validation error
//! #[derive(From, Serialize, Deserialize)]
//! pub enum ValidationError {
//!     PasswordMinLength(PasswordMinLengthError),
//!     InvalidEmail(InvalidEmailError),
//! }
//!
//! // Convenience conversion
//! impl From<(&str, &str)> for ValidationError {
//!     fn from((email, reason): (&str, &str)) -> Self {
//!         Self::InvalidEmail(InvalidEmailError::new(email.into(), reason.into()))
//!     }
//! }
//!
//! // Define a typed vector wrapper for ValidationErrors
//! vec_of_enum::define!(
//!     #[derive(Serialize, Deserialize)]
//!     pub struct ValidationErrors(Vec<ValidationError>);
//! );
//!
//! // Define a typed vector wrapper that also automatically converts from variant types
//! vec_of_enum::define!(
//!     #[derive(Serialize, Deserialize)]
//!     pub struct ValidationErrorsWithVariants(Vec<ValidationError>);
//!     variants = [PasswordMinLengthError, InvalidEmailError];
//! );
//!
//! let mut errors = ValidationErrors::default();
//!
//! // ❌ Without `vec-of-enum`: too verbose
//! errors.push(ValidationError::InvalidEmail(InvalidEmailError::new("user@example.com".into(), "domain is blocked".into())));
//!
//! // ✅ With `vec-of-enum`: very concise
//! errors.push(("user@example.com", "domain is blocked"));
//! ```
//!
//! # Features
//!
//! The wrapper struct created using the `define!` macro:
//!
//! - Is `#[repr(transparent)]` for zero-cost abstraction
//! - Implements `Deref` and `DerefMut` to `Vec<T>` for access to all Vec methods
//! - Provides `new()`, `push()`, and `extend_from()` methods
//! - Implements `Default`, `Extend`, `IntoIterator`, `From<Vec<T>>`, and `Into<Vec<T>>`
//! - Supports automatic conversions from variant types when using the `variants = [...]` option
//!
//! # Custom Derives
//!
//! You can add any derive macros to your struct definition, and they will be applied to
//! the generated struct. For example:
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//! pub enum MyEnum {}
//!
//! vec_of_enum::define!(
//!     #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
//!     pub struct MyVec(Vec<MyEnum>);
//! );
//! ```
//!
//! This allows you to add any necessary derives that your application requires.

#[macro_export]
macro_rules! define {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?;
        $(variants = [$($variant:ty),+];)?
    ) => {
        $crate::define_struct!(
            $(#[$meta])*
            $vis struct $name(Vec<$inner>)
            $(where [$($where_clause)*])?;
        );
        $crate::impl_self!($name, $inner);
        $crate::impl_default!($name);
        $crate::impl_extend!($name, $inner);
        $crate::impl_into_iter_own!($name, $inner);
        $crate::impl_into_iter_ref!($name, $inner);
        $crate::impl_deref!($name, $inner);
        $crate::impl_deref_mut!($name, $inner);
        $crate::impl_from_vec!($name, $inner);
        $crate::impl_into_vec!($name, $inner);
        $($crate::impl_from_value!($name, [$($variant),+]);)?
    };
}

#[macro_export]
macro_rules! define_struct {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident(Vec<$inner:ty>)
        $(where [$($where_clause:tt)*])?;
    ) => {
        #[repr(transparent)]
        $(#[$meta])*
        $vis struct $name(Vec<$inner>)
        $(where $($where_clause)*)?;
    };
}

#[macro_export]
macro_rules! impl_self {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(inner: impl Into<Vec<$inner>>) -> Self {
                Self(inner.into())
            }

            pub fn push(&mut self, value: impl Into<$inner>) {
                self.0.push(value.into())
            }

            pub fn extend_from<T: Into<$inner>>(&mut self, iter: impl IntoIterator<Item = T>) {
                self.extend(iter.into_iter().map(T::into))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_default {
    ($name:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self(Default::default())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_extend {
    ($name:ident, $inner:ty) => {
        impl Extend<$inner> for $name {
            fn extend<I: IntoIterator<Item = $inner>>(&mut self, iter: I) {
                self.0.extend(iter);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_iter_own {
    ($name:ident, $inner:ty) => {
        impl IntoIterator for $name {
            type Item = $inner;
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_iter_ref {
    ($name:ident, $inner:ty) => {
        impl<'a> IntoIterator for &'a $name {
            type Item = &'a $inner;
            type IntoIter = std::slice::Iter<'a, $inner>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref {
    ($name:ident, $inner:ty) => {
        impl std::ops::Deref for $name {
            type Target = Vec<$inner>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_deref_mut {
    ($name:ident, $inner:ty) => {
        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_vec {
    ($name:ident, $inner:ty) => {
        impl From<Vec<$inner>> for $name {
            fn from(vec: Vec<$inner>) -> Self {
                Self(vec)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_from_value {
    ($name:ident, [$($value_source:ty),+]) => {
        $(
            $crate::impl_from_value!($name, $value_source);
        )+
    };
    ($name:ident, $value_source:ty) => {
        impl From<$value_source> for $name {
            fn from(source: $value_source) -> Self {
                Self(vec![source.into()])
            }
        }
    };
}

#[macro_export]
macro_rules! impl_into_vec {
    ($name:ident, $inner:ty) => {
        impl From<$name> for Vec<$inner> {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}
