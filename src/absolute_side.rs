use serde::{Deserialize, Serialize};
use std::{ops, str::FromStr};

/// Describes which player it is
/// ／どちら側のプレイヤーであるかを指定する。
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash, Deserialize, Serialize)]
pub enum AbsoluteSide {
    /// The player whose pieces lie in the A, E and I row when the game starts.
    /// ／A側プレイヤー。初期状態でA, E, Iの三列に渡って自分の駒が配置されている。
    ASide,

    /// The player whose pieces lie in the IA, AU and AI row when the game starts.
    /// ／IA側プレイヤー。初期状態でIA, AU, AIの三列に渡って自分の駒が配置されている。
    IASide,
}

impl FromStr for AbsoluteSide {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(AbsoluteSide::ASide),
            "IA" => Ok(AbsoluteSide::IASide),
            _ => Err(()),
        }
    }
}

impl ops::Not for AbsoluteSide {
    type Output = AbsoluteSide;

    fn not(self) -> Self::Output {
        match self {
            AbsoluteSide::ASide => AbsoluteSide::IASide,
            AbsoluteSide::IASide => AbsoluteSide::ASide,
        }
    }
}
