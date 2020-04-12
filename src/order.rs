//! https://github.com/serde-rs/serde/issues/1158

use indexmap::IndexSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_diff::SerdeDiff;
use std::iter::FromIterator;

/// Order.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, SerdeDiff, Serialize)]
#[serde(untagged)]
pub enum Order {
    #[serde(with = "Unordered")]
    Unordered,
    Ordered(Ordered),
}

impl Default for Order {
    fn default() -> Self {
        Self::Ordered(Ordered::Alphabetic)
    }
}

impl FromIterator<&'static str> for Order {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        Self::Ordered(Ordered::Enumeration(
            iter.into_iter().map(ToString::to_string).collect(),
        ))
    }
}

/// Unordered.
#[derive(Deserialize, Serialize)]
enum Unordered {
    Unordered,
}

impl Unordered {
    fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        <Self as Deserialize>::deserialize(deserializer)?;
        Ok(())
    }

    fn serialize<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&Self::Unordered, serializer)
    }
}

/// Ordered.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, SerdeDiff, Serialize)]
#[serde(untagged)]
pub enum Ordered {
    #[serde(with = "Alphabetic")]
    Alphabetic,
    Enumeration(#[serde_diff(opaque)] IndexSet<String>),
}

/// Alphabetic.
#[derive(Deserialize, Serialize)]
enum Alphabetic {
    Alphabetic,
}

impl Alphabetic {
    fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<(), D::Error> {
        <Self as Deserialize>::deserialize(deserializer)?;
        Ok(())
    }

    fn serialize<S: Serializer>(serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&Self::Alphabetic, serializer)
    }
}
