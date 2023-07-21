#[cfg(feature = "serde_support")]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "serde_support")]
pub trait SerializeAlias: Serialize {}

#[cfg(feature = "serde_support")]
impl<T> SerializeAlias for T where T: Serialize {}

#[cfg(not(feature = "serde_support"))]
pub trait SerializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> SerializeAlias for T {}

#[cfg(feature = "serde_support")]
pub trait DeserializeAlias: DeserializeOwned {}

#[cfg(feature = "serde_support")]
impl<T> DeserializeAlias for T where T: DeserializeOwned {}

#[cfg(not(feature = "serde_support"))]
pub trait DeserializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> DeserializeAlias for T {}
