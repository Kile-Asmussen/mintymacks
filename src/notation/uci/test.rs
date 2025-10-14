use std::fmt::Debug;

use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::{
    bits::{Bits, BoardMask},
    model::{BoardFile, BoardRank, ChessPiece, Square},
    notation::uci::{
        LongAlg, Uci,
        engine::*,
        find_literal_uci,
        gui::{GoCommand, OptVal, Registration, TimeControl, UciGui},
        next_uci_token, parse_uci,
    },
};

use super::engine;

fn roundtrip_render_parse_uci_engine(val: UciEngine) {
    assert_eq!(
        UciEngine::from_string(val.to_string()),
        val.clone(),
        "{}",
        val.to_string()
    )
}

fn roundtrip_render_parse_uci_gui(val: UciGui) {
    assert_eq!(
        UciGui::from_str(val.to_string()),
        Ok(val.clone()),
        "{}",
        val.to_string()
    )
}

fn insert_random_errors<U: Uci + Clone + Examples + PartialEq + Debug, F: FnMut(&U) -> bool>(
    rng: &mut SmallRng,
    mut skip: F,
) {
    for ex in U::examples() {
        if skip(&ex) {
            continue;
        }
        for _ in 0..10 {
            let mut test = vec![];
            ex.clone().print(&mut test);
            for _ in 0..rng.random_range(1..=3) {
                test.insert(rng.random_range(0..test.len()), "XXXX".to_string());
            }
            let x = test.join(" ");
            let mest = test.iter().map(|x| &x[..]).collect::<Vec<_>>();
            assert_eq!(
                Some(ex.clone()),
                parse_uci(&mest[..]).map(|(x, _)| x),
                "{}",
                x
            );
        }
    }
}

#[test]
fn roundtrip() {
    for ue in UciEngine::examples() {
        roundtrip_render_parse_uci_engine(ue)
    }

    for ug in UciGui::examples() {
        roundtrip_render_parse_uci_gui(ug)
    }
}

#[test]
fn resilience_tests() {
    insert_random_errors::<UciEngine, _>(
        &mut SmallRng::from_seed(*b"3.141592653589793238462643383279"),
        |x| match x {
            UciEngine::Id(_) => true,
            _ => false,
        },
    );

    insert_random_errors::<UciGui, _>(
        &mut SmallRng::from_seed(*b"3.141592653589793238462643383279"),
        |x| match x {
            UciGui::Register(_) => true,
            UciGui::SetOption(_, _) => true,
            _ => false,
        },
    );
}

trait Examples: Sized {
    fn examples() -> Vec<Self>;
}

impl Examples for UciEngine {
    fn examples() -> Vec<Self> {
        use UciEngine::*;
        let mut res = vec![UciOk(), ReadyOk()];

        for ar in AuthResult::examples() {
            res.push(CopyProtection(ar));
            res.push(Registration(ar));
        }

        for bm in engine::BestMove::examples() {
            res.push(BestMove(bm));
        }

        for id in IdString::examples() {
            res.push(Id(id));
        }

        for id in InfoString::examples() {
            res.push(Info(vec![id]));
        }

        for ids in InfoString::examples().chunks_exact(2) {
            res.push(Info(ids.into_iter().map(Clone::clone).collect::<Vec<_>>()));
        }

        for ids in InfoString::examples().chunks_exact(3) {
            res.push(Info(ids.into_iter().map(Clone::clone).collect::<Vec<_>>()));
        }

        for ids in InfoString::examples().chunks_exact(10) {
            res.push(Info(ids.into_iter().map(Clone::clone).collect::<Vec<_>>()));
        }

        res
    }
}

impl Examples for AuthResult {
    fn examples() -> Vec<Self> {
        use AuthResult::*;
        vec![Checking, Error, Ok]
    }
}

impl Examples for BestMove {
    fn examples() -> Vec<Self> {
        let mut moves = LongAlg::examples();
        let mut res = vec![];

        while moves.len() >= 2 {
            res.push(BestMove {
                best: moves.pop().unwrap(),
                ponder: moves.pop(),
            });
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
                    && dst.file_rank().0 == BoardFile::F)
                    || (dst.file_rank().1 == BoardRank::_8
                        && orig.file_rank().1 == BoardRank::_7
                        && dst.file_rank().0 == orig.file_rank().0
                        && dst.file_rank().0 == BoardFile::C)
                {
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
            EngineOption {
                name: "Hash".to_string(),
                option_type: OptionType::Spin(SpinType {
                    default: 1024,
                    min: 16,
                    max: 1024 * 1024,
                    value: None,
                }),
            },
            EngineOption {
                name: "OwnBook".to_string(),
                option_type: OptionType::Check(CheckType {
                    default: false,
                    value: None,
                }),
            },
            EngineOption {
                name: "UCI_Opponent".to_string(),
                option_type: OptionType::String(StringType {
                    default: "FM 2882 human Magnus Carlsen".to_string(),
                    value: None,
                }),
            },
            EngineOption {
                name: "My Combo".to_string(),
                option_type: OptionType::Combo(ComboType {
                    default: "foo".to_string(),
                    value: None,
                    variants: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
                }),
            },
        ]
    }
}

impl Examples for InfoString {
    fn examples() -> Vec<Self> {
        use InfoString::*;
        let mut res = vec![];
        res.push(Depth(0));
        res.push(Depth(99));

        res.push(SelDepth(0));
        res.push(SelDepth(99));

        res.push(Time(0));
        res.push(Time(9999));

        res.push(Nodes(0));
        res.push(Nodes(9999));

        res.push(MultiVariation(1));
        res.push(MultiVariation(3));

        let mut line = LongAlg::examples();
        res.push(PrincipleVariation(line[5..15].iter().map(|x| *x).collect()));

        for sb in ScoreBound::examples() {
            for ss in ScoreString::examples() {
                res.push(Score(sb, ss))
            }
        }

        for mv in &line[10..20] {
            res.push(CurrentMove(*mv))
        }

        res.push(CurrentMoveNumber(1));
        res.push(CurrentMoveNumber(3));

        res.push(HashFullPermill(200));
        res.push(HashFullPermill(599));

        res.push(NodesPerSecond(1000));
        res.push(NodesPerSecond(523463));

        res.push(TableBaseHits(0));
        res.push(TableBaseHits(99));

        res.push(ShredderTableBaseHits(0));
        res.push(ShredderTableBaseHits(5));

        res.push(CpuLoadPermill(5));
        res.push(CpuLoadPermill(99));

        res.push(Refutation(
            line[0],
            line[10..20].iter().map(|x| *x).collect(),
        ));
        res.push(CurrLine(2, line[20..25].iter().map(|x| *x).collect()));

        res
    }
}

impl Examples for ScoreBound {
    fn examples() -> Vec<Self> {
        use ScoreBound::*;
        vec![Upper, Lower, Precise]
    }
}

impl Examples for ScoreString {
    fn examples() -> Vec<Self> {
        use ScoreString::*;
        vec![Centipawns(-1000), Centipawns(1000), MateIn(1), MateIn(11)]
    }
}

impl Examples for UciGui {
    fn examples() -> Vec<Self> {
        use UciGui::*;
        let mut res = vec![
            Uci(),
            IsReady(),
            UciNewGame(),
            PonderHit(),
            Quit(),
            Stop(),
            Register(Registration::Later()),
            Register(Registration::NameCode(
                "hello there".to_string(),
                "benkenobi".to_string(),
            )),
            Debug(false),
            Debug(true),
        ];

        for opt in OptVal::examples() {
            res.push(SetOption("spam".to_string(), opt.clone()));
            res.push(SetOption("spam parrot".to_string(), opt));
        }

        for go in GoCommand::examples() {
            res.push(Go(go))
        }

        res
    }
}

impl Examples for OptVal {
    fn examples() -> Vec<Self> {
        vec![
            OptVal::Button(),
            OptVal::StringOrCombo("foo bar".to_string()),
            OptVal::StringOrCombo("baz".to_string()),
            OptVal::Spin(22),
        ]
    }
}

impl Examples for GoCommand {
    fn examples() -> Vec<Self> {
        use GoCommand::*;
        let mut res = vec![
            SearchMoves(LongAlg::examples()),
            Ponder(),
            Depth(1),
            Depth(10),
            Nodes(100),
            Nodes(10000),
            Mate(1),
            Mate(10),
            Infinite(),
            Perft(None),
            Perft(Some(10)),
        ];

        for tc in TimeControl::examples() {
            res.push(Time(tc));
        }

        res
    }
}

impl Examples for TimeControl {
    fn examples() -> Vec<Self> {
        vec![
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 000,
                binc: 000,
                moves_to_go: 00,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 100,
                binc: 000,
                moves_to_go: 00,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 000,
                binc: 100,
                moves_to_go: 00,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 100,
                binc: 100,
                moves_to_go: 00,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 000,
                binc: 000,
                moves_to_go: 10,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 100,
                binc: 000,
                moves_to_go: 10,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 000,
                binc: 100,
                moves_to_go: 10,
            },
            TimeControl {
                wtime: 100,
                btime: 100,
                winc: 100,
                binc: 100,
                moves_to_go: 10,
            },
        ]
    }
}
