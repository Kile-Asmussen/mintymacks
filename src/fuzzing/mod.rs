pub mod enumerate;
pub mod test;

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    process::Command,
    time::Duration,
};

use anyhow::anyhow;
use serde::de;
use trie_rs::inc_search::Position;

use crate::{
    bits::board::BitBoard,
    deque,
    engine::EngineHandle,
    eprintln_async,
    model::{
        ChessPiece,
        moves::{ChessMove, PseudoMove},
    },
    notation::{
        fen::render_fen6,
        uci::{
            engine::{InfoString, UciEngine},
            gui::{GoCommand, PositionString, UciGui},
        },
    },
    tree_map,
};

pub async fn stockfish_perft(
    engine: &mut EngineHandle,
    startpos: Option<&BitBoard>,
    moves: &[ChessMove],
    depth: usize,
) -> tokio::io::Result<BTreeMap<(PseudoMove, Option<ChessPiece>), usize>> {
    let mut res = tree_map! {};

    let pmoves = moves.iter().map(|cm| cm.simplify()).collect();

    let startpos = if let Some(b) = startpos {
        PositionString::Fen(render_fen6(&b))
    } else {
        PositionString::Startpos()
    };

    let mut output = vec![];
    engine
        .interleave_until(
            &mut deque![
                UciGui::Position(startpos, pmoves),
                UciGui::Go(GoCommand::Perft(Some(depth as u64)))
            ],
            &mut output,
            |c| match c {
                UciEngine::Info(v) => match &v[..] {
                    [InfoString::String(s)] => s.starts_with("Nodes searched:"),
                    _ => false,
                },
                _ => false,
            },
            Duration::from_millis(1000),
        )
        .await?;

    for line in output.into_iter().filter_map(infostrings) {
        let split = line.split(":").collect::<Vec<_>>();
        if let [mv, n] = split[..] {
            if let Some(mv) = PseudoMove::parse(mv)
                && let Ok(n) = usize::from_str_radix(n.trim(), 10)
            {
                res.insert(mv, n);
            }
        }
    }

    return Ok(res);

    fn infostrings(uci: UciEngine) -> Option<String> {
        match uci {
            UciEngine::Info(mut v) => match v.pop()? {
                InfoString::String(s) => Some(s),
                _ => None,
            },
            _ => None,
        }
    }
}
