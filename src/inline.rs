use itertools::Itertools;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    convert::TryFrom,
    fmt::{self, Formatter},
};
use toml_edit::{InlineTable, Item, Table, Value};

/// Inline.
pub trait Inline {
    fn inline(&mut self, kind: &Kind);
}

impl Inline for Item {
    fn inline(&mut self, kind: &Kind) {
        *self = inline_item(self, kind);
    }
}

fn inline_item(item: &Item, kind: &Kind) -> Item {
    match kind {
        // TODO:
        Kind::Auto => item.clone(),
        never @ Kind::Manual(None) => match item {
            Item::None => Item::None,
            Item::Value(value) => inline_value(value, never),
            Item::Table(table) => inline_table(table, never),
            _ => unimplemented!(),
            // Item::ArrayOfTables(array_of_tables) => inline_array_of_tables(array_of_tables, never),
        },
        always @ Kind::Manual(Some(0)) => match item {
            Item::None => Item::None,
            Item::Value(value) => inline_value(value, always),
            Item::Table(table) => inline_table(table, always),
            _ => unimplemented!(),
            // Item::ArrayOfTables(array_of_tables) => inline_array_of_tables(array_of_tables, always),
        },
        kind  => {
            // let kind = Kind::Manual(Some(level.saturating_sub(1)));
            match item {
                Item::None => Item::None,
                Item::Value(value) => inline_value(value, &kind),
                Item::Table(table) => inline_table(table, &kind),
                _ => unimplemented!(),
                // Item::ArrayOfTables(array_of_tables) => {
                //     inline_array_of_tables(array_of_tables, kind)
                // }
            }
        }
    }
}

fn inline_value(value: &Value, kind: &Kind) -> Item {
    match kind {
        // TODO:
        Kind::Auto => Item::Value(value.clone()),
        never @ Kind::Manual(None) => match value {
            // Value::Array(array) => {
            //     // let mut array_of_tables = ArrayOfTables::default();
            //     if array.len() == 0 {
            //         return Item::Value(Value::Array(array.clone()));
            //     }
            //     let iter = array.iter();
            //     let t = iter.next().and_then(|v| v.is_value());
            //     let mut buffer = Vec::new();
            //     for value in array.iter() {
            //         let item = inline_value(value, never);
            //         buffer.push(value)
            //     }
            // }
            Value::InlineTable(inline_table) => {
                let mut table = Table::default();
                for (key, value) in inline_table.iter() {
                    *table.entry(key) = inline_value(value, never);
                }
                Item::Table(table)
            }
            value => Item::Value(value.clone()),
        },
        _always @ Kind::Manual(Some(0)) => Item::Value(value.clone()),
        Kind::Manual(Some(level)) => {
            let kind = Kind::Manual(Some(level.saturating_sub(1)));
            match value {
                Value::InlineTable(inline_table) => {
                    let mut table = Table::default();
                    for (key, value) in inline_table.iter() {
                        *table.entry(key) = inline_value(value, &kind);
                    }
                    Item::Table(table)
                }
                value => Item::Value(value.clone()),
            }
        }
    }
}

fn inline_table(input_table: &Table, kind: &Kind) -> Item {
    match kind {
        // TODO:
        Kind::Auto => Item::Table(input_table.clone()),
        never @ Kind::Manual(None) => {
            let mut output_table = Table::default();
            for (key, item) in input_table.iter() {
                *output_table.entry(key) = inline_item(item, never);
            }
            Item::Table(output_table)
        }
        always @ Kind::Manual(Some(0)) => {
            let mut inline_table = InlineTable::default();
            for (key, item) in input_table.iter() {
                if let Item::Value(value) = inline_item(item, always) {
                    inline_table.get_or_insert(key, value);
                } else {
                    panic!("inline item is not value");
                }
            }
            Item::Value(Value::InlineTable(inline_table))
        }
        Kind::Manual(Some(level)) => {
            let kind = Kind::Manual(Some(level.saturating_sub(1)));
            let mut output_table = Table::default();
            for (key, item) in input_table.iter() {
                *output_table.entry(key) = inline_item(item, &kind);
            }
            Item::Table(output_table)
        }
    }
}

// fn from_table_to_inline_table(from: &Table, to: &mut InlineTable) {
//     for (key, item) in from.iter() {
//         match item {
//             Item::Value(value) => {
//                 to.get_or_insert(key, value.clone());
//             }
//             Item::Table(table) => {
//                 let mut value = InlineTable::default();
//                 from_table_to_inline_table(table, &mut value);
//                 to.get_or_insert(key, value);
//             }
//             // TODO:
//             _ => unreachable!(),
//         };
//     }
// }

/// Inline kind.
///
/// - Kind::Manual(None) => "Never" (never inline),
/// - Kind::Manual(Some(0)) => "Always" (Inline start with self),
/// - Kind::Manual(Some(1)) => 1 (Inline start with cildren),
/// - Kind::Manual(Some(2)) => 2 (Inline start with cildren of cildren).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Auto,
    Manual(Option<u64>),
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["Auto", "Never", "Always", "1u64.."];

        struct KindVisitor;

        impl<'de> Visitor<'de> for KindVisitor {
            type Value = Kind;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    FIELDS
                        .iter()
                        .format_with(", ", |v, f| f(&format_args!("`{}`", v)))
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "Auto" => Ok(Kind::Auto),
                    "Never" => Ok(Kind::Manual(None)),
                    "Always" => Ok(Kind::Manual(Some(0))),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Kind, E>
            where
                E: de::Error,
            {
                match u64::try_from(value) {
                    Ok(v) => self.visit_u64(v),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Signed(value),
                        &self,
                    )),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Kind, E>
            where
                E: de::Error,
            {
                if value == 0 {
                    return Err(de::Error::invalid_value(
                        de::Unexpected::Unsigned(value),
                        &self,
                    ));
                }
                Ok(Kind::Manual(Some(value)))
            }
        }

        deserializer.deserialize_any(KindVisitor)
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_str("Auto"),
            Self::Manual(None) => serializer.serialize_str("Never"),
            Self::Manual(Some(0)) => serializer.serialize_str("Always"),
            Self::Manual(Some(v)) => serializer.serialize_u64(*v),
        }
    }
}

#[test]
fn test() {}
