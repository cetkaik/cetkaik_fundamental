use crate::{serialize_color, Color, Profession};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

/// A shortcut macro for creating `ColorAndProf`, which is essentially a tuple of the color and the profession.
/// ／`ColorAndProf` を楽に構築するためのマクロ。
///
/// # Example
/// ```
/// use cetkaik_fundamental::{cp, color, prof, ColorAndProf, Color, Profession};
/// use cetkaik_fundamental::Color::*;
/// use cetkaik_fundamental::Profession::*;
/// assert_eq!(cp!('赤', '兵'), ColorAndProf { color: Kok1, prof: Kauk2 });
/// assert_eq!(cp!('黒', '船'), ColorAndProf { color: Huok2, prof: Nuak1 });
/// ```
#[macro_export]
macro_rules! cp {
    ($c:tt, $p:tt) => {
        ColorAndProf {
            prof: prof!($p),
            color: color!($c),
        }
    };
}
/// Describes a piece that is not a Tam2, and hence can be taken and be placed in a hop1zuo1.
/// ／駒のうち、皇以外を表す。これは手駒として存在できる駒でもある。
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct ColorAndProf {
    /// color of the piece／駒の色
    pub color: Color,
    /// profession of the piece／駒の職種
    pub prof: Profession,
}

impl std::fmt::Display for ColorAndProf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            serialize_color(self.color),
            crate::serialize_prof(self.prof)
        )
    }
}

impl TryInto<ColorAndProf> for &str {
    type Error = ();
    fn try_into(self) -> Result<ColorAndProf, Self::Error> {
        Ok(match self {
            "黒兵" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Kauk2,
            },
            "赤兵" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Kauk2,
            },
            "黒弓" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Gua2,
            },
            "黒車" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Kaun1,
            },
            "黒虎" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Dau2,
            },
            "黒馬" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Maun1,
            },
            "黒筆" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Kua2,
            },
            "黒巫" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Tuk2,
            },
            "黒将" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Uai1,
            },
            "赤弓" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Gua2,
            },
            "赤車" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Kaun1,
            },
            "赤虎" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Dau2,
            },
            "赤馬" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Maun1,
            },
            "赤筆" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Kua2,
            },
            "赤巫" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Tuk2,
            },
            "赤将" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Uai1,
            },
            "黒王" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Io,
            },
            "赤王" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Io,
            },
            "黒船" => ColorAndProf {
                color: Color::Huok2,
                prof: Profession::Nuak1,
            },
            "赤船" => ColorAndProf {
                color: Color::Kok1,
                prof: Profession::Nuak1,
            },
            _ => return Err(()),
        })
    }
}
