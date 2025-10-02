use crate::{bits::{Bits, BoardMask}, model::{BoardFile, BoardRank, ChessPiece}, notation::uci::{engine::*, literal_uci, token_uci, LongAlg, Uci}};

use super::engine;

fn roundtrip_render_parse_uci_engine(val: UciEngine) {
    assert_eq!(UciEngine::from_str(&val.to_string()), Some(val))
}

fn roundtrip_parse_uci_engine(val: &str) {
    assert_eq!(UciEngine::from_str(val).map(|s| s.to_string()), Some(val.to_string()));
}

#[test]
fn roundtrip_1() {
    for ue in UciEngine::examples() {
        roundtrip_render_parse_uci_engine(ue)
    }
}

trait Examples : Sized {
    fn examples() -> Vec<Self>;
}

impl Examples for UciEngine {
    fn examples() -> Vec<Self> {
        use UciEngine::*;
        let mut res = vec![
            UciOk(),
            ReadyOk(),
        ];

        for ar in AuthResult::examples() {
            res.push(CopyProtection(ar))
        }

        for bm in engine::BestMove::examples() {
            res.push(BestMove(bm))
        }

        for id in IdString::examples() {
            res.push(Id(id));
        }

        for opt in EngineOption::examples() {
            res.push(Option(opt))
        }

        res
    }
}

impl Examples for AuthResult {
    fn examples() -> Vec<Self> {
        use AuthResult::*;
        vec![
            Checking, Error, Ok
        ]
    }
}

impl Examples for BestMove {
    fn examples() -> Vec<Self> {
        let mut moves = LongAlg::examples();
        let mut res = vec![];

        while moves.len() >= 2 {
            res.push(
                BestMove {
                    best: moves.pop().unwrap(),
                    ponder: moves.pop(),
                }
            );
        }

        res
    }
}

impl Examples for IdString {
    fn examples() -> Vec<Self> {
        use IdString::*;
        vec![
            Name("foo".to_string()),
            Name("foo bar".to_string()),

            Author("foo".to_string()),
            Author("foo bar".to_string()),
        ]
    }
}

impl Examples for LongAlg {
    fn examples() -> Vec<Self> {
        let mut res = vec![];
        let mut n = 0;
        for orig in Bits(BoardMask::MAX) {
            for dst in Bits(BoardMask::MAX) {
                if dst == orig {
                    continue;
                }
                n += 1;
                if n % 100 == 0 {
                    res.push((orig.to(dst), None));
                }

                if (dst.file_rank().1 == BoardRank::_1
                && orig.file_rank().1 == BoardRank::_2
                && dst.file_rank().0 == orig.file_rank().0
                && dst.file_rank().0 == BoardFile::F )
                || (dst.file_rank().1 == BoardRank::_8
                && orig.file_rank().1 == BoardRank::_7
                && dst.file_rank().0 == orig.file_rank().0
                && dst.file_rank().0 == BoardFile::C ) {
                    res.push((orig.to(dst), None));
                    res.push((orig.to(dst), Some(ChessPiece::Queen)));
                    res.push((orig.to(dst), Some(ChessPiece::Rook)));
                    res.push((orig.to(dst), Some(ChessPiece::Bishop)));
                    res.push((orig.to(dst), Some(ChessPiece::Knight)));
                }
            }
        }

        res
    }
}

impl Examples for EngineOption {
    fn examples() -> Vec<Self> {
        vec![
            EngineOption { name: "Hash".to_string(), option_type: OptionType::Spin(SpinType { default: 1024, min: 16, max: 1024 * 1024, value: None }) },
            EngineOption { name: "OwnBook".to_string(), option_type: OptionType::Check(CheckType { default: false, value: None }) },
            EngineOption { name: "UCI_Opponent".to_string(), option_type: OptionType::String(StringType { default: "FM 2882 human Magnus Carlsen".to_string(), value: None }) },
            EngineOption { name: "MyCombo".to_string(), option_type: OptionType::Combo(ComboType {default:"foo".to_string(),value:None, variants: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()] }) }
        ]
    }
}