use itertools::Itertools;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_diff::SerdeDiff;
use std::{
    convert::TryFrom,
    fmt::{self, Formatter},
};

/// Inline.
///
/// - `Inline::Auto` - TODO:,
/// - `Inline::Manual(None)` => never inline ("None"),
/// - `Inline::Manual(Some(0))` => inline starting with self (level 0),
/// - `Inline::Manual(Some(1))` => inline starting with children (level 1),
/// - `Inline::Manual(Some(2))` => inline starting with children of children (level 2),
/// - etc.
#[derive(Clone, Copy, Debug, Eq, PartialEq, SerdeDiff)]
pub enum Inline {
    Auto,
    Manual(Option<u64>),
}

impl Inline {
    pub fn branch(&self) -> Self {
        match *self {
            Self::Manual(Some(level)) => Self::Manual(Some(level.saturating_sub(1))),
            mode => mode,
        }
    }

    pub fn level(&self, level: usize) -> Self {
        let mut inline = *self;
        for _ in 0..level {
            inline = inline.branch();
        }
        inline
    }

    pub fn is_inline(&self) -> bool {
        match self {
            Self::Auto => true,
            Self::Manual(Some(0)) => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for Inline {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        const FIELDS: &'static [&'static str] = &["Auto", "None", "0.."];

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Inline;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    FIELDS
                        .iter()
                        .format_with(", ", |v, f| f(&format_args!("`{}`", v)))
                )
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                match value {
                    "Auto" => Ok(Inline::Auto),
                    "None" => Ok(Inline::Manual(None)),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }

            fn visit_i64<E: de::Error>(self, value: i64) -> Result<Inline, E> {
                match u64::try_from(value) {
                    Ok(v) => self.visit_u64(v),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Signed(value),
                        &self,
                    )),
                }
            }

            fn visit_u64<E: de::Error>(self, value: u64) -> Result<Inline, E> {
                Ok(Inline::Manual(Some(value)))
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for Inline {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("Auto"),
            Self::Manual(None) => serializer.serialize_str("None"),
            Self::Manual(Some(v)) => serializer.serialize_u64(*v),
        }
    }
}
