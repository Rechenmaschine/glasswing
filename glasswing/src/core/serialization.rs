#[cfg(feature = "serde_support")]
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "serde_support")]
pub use Serialize as SerializeAlias;

#[cfg(not(feature = "serde_support"))]
pub trait SerializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> SerializeAlias for T {}

#[cfg(feature = "serde_support")]
pub use DeserializeOwned as DeserializeAlias;

#[cfg(not(feature = "serde_support"))]
pub trait DeserializeAlias {}

#[cfg(not(feature = "serde_support"))]
impl<T> DeserializeAlias for T {}