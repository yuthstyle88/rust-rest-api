//! Automatic serialization and deserialization support.

#[doc(inline)]
pub use serde::ser::{Serialize, Serializer};

#[doc(inline)]
pub use serde::de::{Deserialize, DeserializeOwned, Deserializer};

#[doc(hidden)]
pub use serde::*;
