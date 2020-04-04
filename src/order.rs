use indexmap::IndexMap;
use itertools::Itertools;
use serde::{
    de,
    de::{SeqAccess, Visitor},
    ser::SerializeSeq,
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_diff::SerdeDiff;
use std::{
    borrow::Cow,
    cmp::{Ordering, Reverse},
    fmt::{self, Formatter},
};
use toml_edit::{Array, InlineTable, Item, Table, Value};

fn cmp(kind: &Kind, a: &str, b: &str) -> Ordering {
    match kind {
        Kind::Direction(Direction::Forward) => Ord::cmp(a, b),
        Kind::Direction(Direction::Backward) => Ord::cmp(&Reverse(a), &Reverse(b)),
        Kind::Enumeration(map) => match (map.get(a), map.get(b)) {
            (Some(a), Some(b)) => Ord::cmp(a, b),
            (a, b) => Ord::cmp(&Reverse(a), &Reverse(b)),
        },
    }
}

/// Order.
pub trait Order {
    fn order(&mut self, kind: &Kind);
}

impl Order for Item {
    fn order(&mut self, kind: &Kind) {
        match self {
            Item::Value(Value::Array(array)) => {
                array.order(kind);
            }
            Item::Value(Value::InlineTable(inline_table)) => {
                inline_table.order(kind);
            }
            Item::Table(table) => {
                table.order(kind);
            }
            _ => return,
        }
    }
}

impl Order for Array {
    fn order(&mut self, kind: &Kind) {
        let sorted = self
            .iter()
            .sorted_by(|l, r| match (l.as_str(), r.as_str()) {
                (Some(l), Some(r)) => cmp(kind, l, r),
                _ => Ordering::Equal,
            });
        let mut array = Array::default();
        for value in sorted {
            array.push(value.clone());
        }
        *self = array;
    }
}

impl Order for InlineTable {
    fn order(&mut self, kind: &Kind) {
        let sorted = self.iter().sorted_by(|a, b| cmp(kind, &a.0, &b.0));
        let mut inline_table = InlineTable::default();
        for (key, value) in sorted {
            inline_table.get_or_insert(key, value.clone());
        }
        *self = inline_table;
    }
}

impl Order for Table {
    fn order(&mut self, kind: &Kind) {
        let sorted = self.iter().sorted_by(|a, b| cmp(kind, &a.0, &b.0));
        let mut table = Table::default();
        for (key, item) in sorted {
            *table.entry(key) = item.clone();
        }
        *self = table;
    }
}

// TODO: Add SerdeDiff (https://github.com/amethyst/serde-diff/pull/17)
/// Order kind.
#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Direction(Direction),
    Enumeration(IndexMap<Cow<'static, str>, usize>),
}

impl<'de> Deserialize<'de> for Kind {
    fn deserialize<D>(deserializer: D) -> Result<Kind, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["Forward", "Backward"];

        struct OrderVisitor;

        impl<'de> Visitor<'de> for OrderVisitor {
            type Value = Kind;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(
                    formatter,
                    r#"{} or `["...", "..." ...]`"#,
                    FIELDS
                        .iter()
                        .format_with(", ", |v, f| f(&format_args!("`{}`", v)))
                )
            }

            fn visit_str<E>(self, value: &str) -> Result<Kind, E>
            where
                E: de::Error,
            {
                match value {
                    "Forward" => Ok(Kind::Direction(Direction::Forward)),
                    "Backward" => Ok(Kind::Direction(Direction::Backward)),
                    _ => Err(de::Error::unknown_field(value, FIELDS)),
                }
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Kind, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut map = IndexMap::with_capacity(seq.size_hint().unwrap_or(0));
                let mut index = 0;
                while let Some(key) = seq.next_element()? {
                    map.insert(key, index);
                    index += 1;
                }
                Ok(Kind::Enumeration(map))
            }
        }

        deserializer.deserialize_any(OrderVisitor)
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Direction(direction) => direction.serialize(serializer),
            Self::Enumeration(map) => {
                let mut seq = serializer.serialize_seq(Some(map.len()))?;
                for key in map.keys() {
                    seq.serialize_element(key)?;
                }
                seq.end()
            }
        }
    }
}

// impl SerdeDiff for Kind {
//     fn name();
// }

impl Default for Kind {
    fn default() -> Self {
        Self::Direction(Direction::Forward)
    }
}

/// Direction.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Direction {
    Forward,
    Backward,
}
