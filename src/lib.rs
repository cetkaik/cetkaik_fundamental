//! The most fundamental data types and whatnot for cetkaik, a board game. See <https://sites.google.com/view/cet2kaik/the-standardized-rule-in-english> for more context.
//! ／机戦（セットカイク）のための最も基本的なデータ型など。
#![warn(clippy::pedantic, clippy::nursery, missing_docs)]
#![allow(
    clippy::non_ascii_literal,
    clippy::use_self,
    clippy::upper_case_acronyms,
    clippy::module_name_repetitions
)]

/// Denotes the color of a piece／駒の色を表す。
pub mod color;

/// Denotes the profession of a piece／駒の職業を表す。
pub mod profession;

/// Denotes the combination of a color and a profession. / 駒の色と職業を表す。 
pub mod color_and_prof;

/// Denotes the absolute side: `ASide` vs. `IASide` / 絶対座標での陣営 (`ASide` vs. `IASide`) を表す。
pub mod absolute_side;

pub use color::*;
pub use color_and_prof::*;
pub use profession::*;
pub use absolute_side::*;
