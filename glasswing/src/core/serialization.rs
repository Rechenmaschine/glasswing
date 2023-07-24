//! # Aliases for serialization
//!
//! Used for conditional compilation of the serialization features.
//!
//! - If the `serde_support` feature is **enabled**, `SerializeAlias` and `DeserializeAlias`
//! are traits that are implemented for all types that implement `Serialize` and `DeserializeOwned`.
//!
//! - If the `serde_support` feature is **disabled**, `SerializeAlias` and `DeserializeAlias`
//! are empty traits that are implemented for all types.

#[cfg(feature = "serde_support")]
use serde::{de::DeserializeOwned, Serialize};

/// `SerializeAlias` is an alias for `serde::Serialize` if the `serde_support` feature
/// is enabled. Otherwise, it is an empty trait implemented for all types.
#[cfg(feature = "serde_support")]
pub trait SerializeAlias: Serialize {}

#[cfg(feature = "serde_support")]
impl<T> SerializeAlias for T where T: Serialize {}

/// `SerializeAlias` is an alias for `serde::Serialize` if the `serde_support` feature
/// is enabled. Otherwise, it is an empty trait implemented for all types.
#[cfg(not(feature = "serde_support"))]
pub trait SerializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> SerializeAlias for T {}

/// `DeserializeAlias` is an alias for `serde::DeserializeOwned` if the `serde_support` feature
/// is enabled. Otherwise, it is an empty trait implemented for all types.
#[cfg(feature = "serde_support")]
pub trait DeserializeAlias: DeserializeOwned {}

#[cfg(feature = "serde_support")]
impl<T> DeserializeAlias for T where T: DeserializeOwned {}

/// `DeserializeAlias` is an alias for `serde::DeserializeOwned` if the `serde_support` feature
/// is enabled. Otherwise, it is an empty trait implemented for all types.
#[cfg(not(feature = "serde_support"))]
pub trait DeserializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> DeserializeAlias for T {}
