/// Denotes the color of a piece／駒の色を表す。
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub enum Color {
    /// Red, 赤
    Kok1,

    /// Black, 黒
    Huok2,
}

/// Serializes [`Color`](./enum.Color.html).／[`Color`](./enum.Color.html)を文字列に変換する。
/// # Examples
/// ```
/// use cetkaik_fundamental::*;
///
/// assert_eq!(serialize_color(Color::Kok1), "赤");
/// assert_eq!(serialize_color(Color::Huok2), "黒");
/// ```
///
#[must_use]
pub const fn serialize_color(color: Color) -> &'static str {
    match color {
        Color::Huok2 => "黒",
        Color::Kok1 => "赤",
    }
}

use std::str::FromStr;
impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match &*s {
            "red" | "赤" | "kok1" | "红" | "紅" => Ok(Color::Kok1),
            "black" | "黒" | "huok2" | "黑" => Ok(Color::Huok2),
            _ => Err(()),
        }
    }
}
impl serde::ser::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(serialize_color(*self))
    }
}
struct ColorVisitor;

impl<'de> serde::de::Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a color")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Color::from_str(s).map_or_else(
            |_| {
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(s),
                    &self,
                ))
            },
            |c| Ok(c),
        )
    }
}

impl<'de> serde::de::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}

/// A shortcut macro for creating `Color`.
/// ／`Color` を楽に構築するためのマクロ。
#[macro_export]
macro_rules! color {
    ('黒') => {
        Color::Huok2
    };
    ('赤') => {
        Color::Kok1
    };
}
