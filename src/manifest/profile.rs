use indexmap::IndexMap;
use itertools::Itertools;
use serde::{de, de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    convert::TryFrom,
    fmt::{self, Formatter},
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "kebab-case")]
pub struct Profile {
    #[serde(flatten)]
    built_in: BuiltIn,
    user: IndexMap<String, Settings>,
}

impl Profile {
    pub fn dev(&self) -> Option<&Settings> {
        self.built_in.dev.as_ref()
    }

    pub fn dev_mut(&mut self) -> Option<&mut Settings> {
        self.built_in.dev.as_mut()
    }

    pub fn release(&self) -> Option<&Settings> {
        self.built_in.release.as_ref()
    }

    pub fn release_mut(&mut self) -> Option<&mut Settings> {
        self.built_in.release.as_mut()
    }

    pub fn test(&self) -> Option<&Settings> {
        self.built_in.test.as_ref()
    }

    pub fn test_mut(&mut self) -> Option<&mut Settings> {
        self.built_in.test.as_mut()
    }

    pub fn bench(&self) -> Option<&Settings> {
        self.built_in.bench.as_ref()
    }

    pub fn bench_mut(&mut self) -> Option<&mut Settings> {
        self.built_in.bench.as_mut()
    }

    pub fn doc(&self) -> Option<&Settings> {
        self.built_in.doc.as_ref()
    }

    pub fn doc_mut(&mut self) -> Option<&mut Settings> {
        self.built_in.doc.as_mut()
    }
}

impl Index<&str> for Profile {
    type Output = Settings;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "dev" => self.built_in.dev.as_ref().unwrap(),
            "release" => self.built_in.release.as_ref().unwrap(),
            "test" => self.built_in.test.as_ref().unwrap(),
            "bench" => self.built_in.bench.as_ref().unwrap(),
            "doc" => self.built_in.doc.as_ref().unwrap(),
            index => &self.user[index],
        }
    }
}

impl IndexMut<&str> for Profile {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "dev" => self.built_in.dev.as_mut().unwrap(),
            "release" => self.built_in.release.as_mut().unwrap(),
            "test" => self.built_in.test.as_mut().unwrap(),
            "bench" => self.built_in.bench.as_mut().unwrap(),
            "doc" => self.built_in.doc.as_mut().unwrap(),
            index => &mut self.user[index],
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct BuiltIn {
    dev: Option<Settings>,
    release: Option<Settings>,
    test: Option<Settings>,
    bench: Option<Settings>,
    doc: Option<Settings>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    opt_level: Option<OptLevel>,
    debug: Option<Debug>,
    debug_assertions: Option<bool>,
    overflow_checks: Option<bool>,
    lto: Option<Lto>,
    panic: Option<Panic>,
    incremental: Option<bool>,
    codegen_units: Option<u64>,
    rpath: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OptLevel {
    Zero,
    One,
    Two,
    Three,
    S,
    Z,
}

impl<'de> Deserialize<'de> for OptLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["0", "1", "2", "3", "s", "z"];

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = OptLevel;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    FIELDS
                        .iter()
                        .format_with(", ", |v, f| f(&format_args!("`{}`", v)))
                )
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
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

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(OptLevel::Zero),
                    1 => Ok(OptLevel::One),
                    2 => Ok(OptLevel::Two),
                    3 => Ok(OptLevel::Three),
                    _ => Err(de::Error::invalid_value(Unexpected::Unsigned(value), &self)),
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "s" => Ok(OptLevel::S),
                    "z" => Ok(OptLevel::Z),
                    _ => Err(de::Error::invalid_value(Unexpected::Str(value), &self)),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for OptLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Zero => 0.serialize(serializer),
            Self::One => 1.serialize(serializer),
            Self::Two => 2.serialize(serializer),
            Self::Three => 3.serialize(serializer),
            Self::S => "s".serialize(serializer),
            Self::Z => "z".serialize(serializer),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Debug {
    Bool(bool),
    Enum(DebugEnum),
}

impl From<Debug> for u64 {
    fn from(from: Debug) -> Self {
        match from {
            Debug::Bool(true) => 2,
            Debug::Bool(false) => 0,
            Debug::Enum(v) => v.into(),
        }
    }
}

impl PartialEq for Debug {
    fn eq(&self, other: &Self) -> bool {
        u64::from(*self) == (*other).into()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DebugEnum {
    Zero,
    One,
    Two,
}

impl From<DebugEnum> for u64 {
    fn from(from: DebugEnum) -> Self {
        match from {
            DebugEnum::Zero => 0,
            DebugEnum::One => 1,
            DebugEnum::Two => 2,
        }
    }
}

impl<'de> Deserialize<'de> for DebugEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &'static [&'static str] = &["0", "1", "2"];

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = DebugEnum;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "{}",
                    FIELDS
                        .iter()
                        .format_with(", ", |v, f| f(&format_args!("`{}`", v)))
                )
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
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

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(DebugEnum::Zero),
                    1 => Ok(DebugEnum::One),
                    2 => Ok(DebugEnum::Two),
                    _ => Err(de::Error::invalid_value(Unexpected::Unsigned(value), &self)),
                }
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl Serialize for DebugEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Zero => 0.serialize(serializer),
            Self::One => 1.serialize(serializer),
            Self::Two => 2.serialize(serializer),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Lto {
    Bool(bool),
    Enum(LtoEnum),
}

impl PartialEq for Lto {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Lto::Bool(true) | Lto::Enum(LtoEnum::Fat),
                Lto::Bool(true) | Lto::Enum(LtoEnum::Fat),
            ) => true,
            (Lto::Bool(lhs), Lto::Bool(rhs)) => lhs == rhs,
            (Lto::Enum(lhs), Lto::Enum(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum LtoEnum {
    #[serde(rename = "fat")]
    Fat,
    #[serde(rename = "thin")]
    Thin,
    #[serde(rename = "off")]
    Off,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Panic {
    #[serde(rename = "unwind")]
    Unwind,
    #[serde(rename = "abort")]
    Abort,
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use maplit::hashmap;

    macro_rules! settings {
        () => {
            Settings {
                opt_level: Some(OptLevel::Zero),
                debug: Some(Debug::Enum(DebugEnum::Zero)),
                debug_assertions: Some(true),
                overflow_checks: Some(true),
                lto: Some(Lto::Bool(true)),
                panic: Some(Panic::Unwind),
                incremental: Some(true),
                codegen_units: Some(1),
                rpath: Some(true),
            }
        };
    }

    macro_rules! built_in {
        () => {
            BuiltIn {
                dev: Some(settings!()),
                release: Some(settings!()),
                test: Some(settings!()),
                bench: Some(settings!()),
                doc: Some(settings!()),
            }
        };
    }

    #[test]
    fn profile() -> Result<()> {
        // let de = Profile {
        //     built_in: built_in!(),
        //     user: IndexMap::new(),
        // };
        Ok(())
    }

    #[test]
    fn built_in() -> Result<()> {
        let de = built_in!();
        let ser = "[dev]\n\
            opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n\
            \n\
            [release]\n\
            opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n\
            \n\
            [test]\n\
            opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n\
            \n\
            [bench]\n\
            opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n\
            \n\
            [doc]\n\
            opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n";
        assert_eq!(ser, toml::to_string(&de)?);
        assert_eq!(de, toml::from_str(ser)?);
        Ok(())
    }

    #[test]
    fn settings() -> Result<()> {
        let de = settings!();
        let ser = "opt-level = 0\n\
            debug = 0\n\
            debug-assertions = true\n\
            overflow-checks = true\n\
            lto = true\n\
            panic = \"unwind\"\n\
            incremental = true\n\
            codegen-units = 1\n\
            rpath = true\n";
        assert_eq!(ser, toml::to_string(&de)?);
        assert_eq!(de, toml::from_str(ser)?);
        Ok(())
    }

    #[test]
    fn opt_level() -> Result<()> {
        assert_eq!("0", toml::to_string(&OptLevel::Zero)?);
        assert_eq!("1", toml::to_string(&OptLevel::One)?);
        assert_eq!("2", toml::to_string(&OptLevel::Two)?);
        assert_eq!("3", toml::to_string(&OptLevel::Three)?);
        assert_eq!(r#""s""#, toml::to_string(&OptLevel::S)?);
        assert_eq!(r#""z""#, toml::to_string(&OptLevel::Z)?);

        assert_eq!(
            hashmap! { "opt_level" => OptLevel::Zero },
            toml::from_str("opt_level = 0")?
        );
        assert_eq!(
            hashmap! { "opt_level" => OptLevel::One },
            toml::from_str("opt_level = 1")?
        );
        assert_eq!(
            hashmap! { "opt_level" => OptLevel::Two },
            toml::from_str("opt_level = 2")?
        );
        assert_eq!(
            hashmap! { "opt_level" => OptLevel::Three },
            toml::from_str("opt_level = 3")?
        );
        assert_eq!(
            hashmap! { "opt_level" => OptLevel::S },
            toml::from_str(r#"opt_level = "s""#)?
        );
        assert_eq!(
            hashmap! { "opt_level" => OptLevel::Z },
            toml::from_str(r#"opt_level = "z""#)?
        );
        Ok(())
    }

    #[test]
    fn debug() -> Result<()> {
        assert!(Debug::Bool(false) == Debug::Enum(DebugEnum::Zero));
        assert_eq!(Debug::Bool(true), Debug::Enum(DebugEnum::Two));

        assert_eq!("false", toml::to_string(&Debug::Bool(false))?);
        assert_eq!("true", toml::to_string(&Debug::Bool(true))?);
        assert_eq!("0", toml::to_string(&Debug::Enum(DebugEnum::Zero))?);
        assert_eq!("1", toml::to_string(&Debug::Enum(DebugEnum::One))?);
        assert_eq!("2", toml::to_string(&Debug::Enum(DebugEnum::Two))?);

        assert_eq!(
            hashmap! { "debug" => Debug::Enum(DebugEnum::Zero) },
            toml::from_str("debug = 0")?
        );
        assert_eq!(
            hashmap! { "debug" => Debug::Enum(DebugEnum::One) },
            toml::from_str("debug = 1")?
        );
        assert_eq!(
            hashmap! { "debug" => Debug::Enum(DebugEnum::Two) },
            toml::from_str("debug = 2")?
        );
        Ok(())
    }

    #[test]
    fn lto() -> Result<()> {
        assert_eq!("false", toml::to_string(&Lto::Bool(false))?);
        assert_eq!("true", toml::to_string(&Lto::Bool(true))?);
        assert_eq!(r#""fat""#, toml::to_string(&Lto::Enum(LtoEnum::Fat))?);
        assert_eq!(r#""thin""#, toml::to_string(&Lto::Enum(LtoEnum::Thin))?);
        assert_eq!(r#""off""#, toml::to_string(&Lto::Enum(LtoEnum::Off))?);

        assert_eq!(
            hashmap! { "lto" => Lto::Bool(false) },
            toml::from_str("lto = false")?
        );
        assert_eq!(
            hashmap! { "lto" => Lto::Bool(true) },
            toml::from_str("lto = true")?
        );
        assert_eq!(
            hashmap! { "lto" => Lto::Enum(LtoEnum::Fat) },
            toml::from_str(r#"lto = "fat""#)?
        );
        assert_eq!(
            hashmap! { "lto" => Lto::Enum(LtoEnum::Thin) },
            toml::from_str(r#"lto = "thin""#)?
        );
        assert_eq!(
            hashmap! { "lto" => Lto::Enum(LtoEnum::Off) },
            toml::from_str(r#"lto = "off""#)?
        );
        Ok(())
    }

    #[test]
    fn panic() -> Result<()> {
        assert_eq!(r#""unwind""#, toml::to_string(&Panic::Unwind)?);
        assert_eq!(r#""abort""#, toml::to_string(&Panic::Abort)?);

        assert_eq!(Panic::Unwind, toml::from_str(r#""unwind""#)?);
        assert_eq!(Panic::Abort, toml::from_str(r#""abort""#)?);
        Ok(())
    }
}
