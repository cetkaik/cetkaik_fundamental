use serde::{Deserialize, Serialize};
use std::{ops, str::FromStr};

/// Describes which player it is
/// ／どちら側のプレイヤーであるかを指定する。
#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash, Deserialize, Serialize)]
pub enum Side {
    /// The player whose pieces lie in the A, E and I row when the game starts.
    /// ／A側プレイヤー。初期状態でA, E, Iの三列に渡って自分の駒が配置されている。
    ASide,

    /// The player whose pieces lie in the IA, AU and AI row when the game starts.
    /// ／IA側プレイヤー。初期状態でIA, AU, AIの三列に渡って自分の駒が配置されている。
    IASide,
}

impl FromStr for Side {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Side::ASide),
            "IA" => Ok(Side::IASide),
            _ => Err(()),
        }
    }
}

impl ops::Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::ASide => Side::IASide,
            Side::IASide => Side::ASide,
        }
    }
}
