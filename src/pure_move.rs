use crate::{Color, Profession};

/// Describes a move.
/// ／指した手を表す。
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PureMove_<Coord> {
    /// A non-Tam2 piece moves from a square on a board to another square without stepping.
    /// ／皇ではない駒が、盤上から盤上に踏越えなしで移動する。
    NonTamMoveSrcDst {
        /// origin／開始点
        src: Coord,
        /// final destination／終了点
        dest: Coord,
        /// whether a water-entry ciurl is required／入水判定が必要かどうか
        is_water_entry_ciurl: bool,
    },
    /// A non-Tam2 piece moves from a square on a board to another square, during which it steps another piece and does a finite movement.
    /// ／皇ではない駒が、盤上から盤上に踏越えを伴いながら移動し、踏越え後は有限移動をする。
    NonTamMoveSrcStepDstFinite {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// destination／終了点
        dest: Coord,
        /// whether a water-entry ciurl is required／入水判定が必要かどうか
        is_water_entry_ciurl: bool,
    },
    /// A non-Tam2 piece moves from a square on a board to another square, during which it steps another piece and tries to do a directional, infinite movement.
    /// ／皇ではない駒が、盤上から盤上に踏越えを伴いながら移動し、踏越え後は無限移動をしようとする。
    InfAfterStep {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// the planned LOCATION. After casting the sticks, some rules necessitates that you go to the destination or to the direction that you have declared beforehand.
        /// Hence the confusing name.
        /// ／行く予定の場所。踏越え判定後の移動先は、ルールによっては「計画したマス」である必要があったり「計画した方角」である必要があったりする。
        planned_direction: Coord,
    },
    /// A non-Tam2 piece moves from hop1zuo1 to a square on a board.
    /// ／皇ではない駒が、手駒から盤上に移動する。
    NonTamMoveFromHopZuo {
        /// color／駒の色
        color: Color,
        /// profession／駒の役職
        prof: Profession,
        /// destination／終了点
        dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square without stepping.
    /// ／皇が盤上から盤上に踏越えなしで移動する。
    TamMoveNoStep {
        /// origin／開始点
        src: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square. In the former half of its movement, it steps another piece.
    /// ／皇が盤上から盤上に移動し、一回目の移動の最中に踏越えをする。
    TamMoveStepsDuringFormer {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
    /// A Tam2 moves from a square on a board to another square. In the latter half of its movement, it steps another piece.
    /// ／皇が盤上から盤上に移動し、二回目の移動の最中に踏越えをする。
    TamMoveStepsDuringLatter {
        /// origin／開始点
        src: Coord,
        /// via point／経由点
        step: Coord,
        /// first destination／一回目の終了点
        first_dest: Coord,
        /// second destination／二回目の終了点
        second_dest: Coord,
    },
}

impl<Coord: std::fmt::Display> std::fmt::Display for PureMove_<Coord> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PureMove_::InfAfterStep {
                src,
                step,
                planned_direction,
            } => write!(f, "{src}片{step}心{planned_direction}"),
            PureMove_::NonTamMoveFromHopZuo { color, prof, dest } => write!(
                f,
                "{}{}{}",
                crate::serialize_color(*color),
                crate::serialize_prof(*prof),
                (dest)
            ),
            PureMove_::NonTamMoveSrcDst {
                src,
                dest,
                is_water_entry_ciurl,
            } => write!(
                f,
                "{}片{}{}",
                (src),
                (dest),
                if *is_water_entry_ciurl { "水" } else { "" }
            ),
            PureMove_::NonTamMoveSrcStepDstFinite {
                src,
                dest,
                is_water_entry_ciurl,
                step,
            } => write!(
                f,
                "{}片{}{}{}",
                (src),
                (step),
                (dest),
                if *is_water_entry_ciurl { "水" } else { "" }
            ),
            PureMove_::TamMoveNoStep {
                src,
                first_dest,
                second_dest,
            } => write!(f, "{src}皇[{first_dest}]{second_dest}"),
            PureMove_::TamMoveStepsDuringFormer {
                src,
                first_dest,
                second_dest,
                step,
            } => write!(
                f,
                "{}皇{}[{}]{}",
                (src),
                (step),
                (first_dest),
                (second_dest)
            ),
            PureMove_::TamMoveStepsDuringLatter {
                src,
                first_dest,
                second_dest,
                step,
            } => write!(
                f,
                "{}皇[{}]{}{}",
                (src),
                (first_dest),
                (step),
                (second_dest)
            ),
        }
    }
}
