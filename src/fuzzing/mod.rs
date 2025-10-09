pub mod enumerate;
pub mod test;

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    process::Command,
    time::Duration,
};

use anyhow::anyhow;
use trie_rs::inc_search::Position;

use crate::{
    deque,
    engine::EngineHandle,
    eprintln_async,
    model::{
        ChessPiece,
        moves::{ChessMove, PseudoMove},
    },
    notation::uci::{
        engine::{InfoString, UciEngine},
        gui::{GoCommand, PositionString, UciGui},
    },
    tree_map,
};

pub async fn stockfish_perft(
    engine: &mut EngineHandle,
    moves: &[ChessMove],
    depth: usize,
) -> tokio::io::Result<BTreeMap<(PseudoMove, Option<ChessPiece>), usize>> {
    let mut res = tree_map! {};

    let pmoves = moves.iter().map(|cm| cm.simplify()).collect();

    let mut output = vec![];
    engine
        .interleave(
            &mut deque![
                UciGui::Position(PositionString::Startpos(), pmoves),
                UciGui::Go(GoCommand::Perft(Some(depth as u64))),
            ],
            &mut output,
            Duration::from_millis(5) * 2u32.pow(depth as u32),
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
