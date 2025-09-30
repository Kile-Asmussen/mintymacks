pub mod enumerate;
pub mod test;

use std::{collections::HashMap, process::Command};

use anyhow::anyhow;

use crate::model::{
    ChessPiece,
    moves::{ChessMove, PseudoMove},
};

pub fn stockfish_perft(
    moves: &[ChessMove],
    depth: usize,
) -> anyhow::Result<HashMap<(PseudoMove, Option<ChessPiece>), usize>> {
    let mut res = hash_map!{};

    let output = String::from_utf8(
        Command::new("./src/fuzzing/stockfish-perft.sh")
            .arg(
                moves
                    .into_iter()
                    .map(|m| m.longalg())
                    .collect::<Vec<_>>()
                    .join(" "),
            )
            .arg(depth.to_string())
            .output()?
            .stdout,
    )?;

    for line in output.trim_end().lines() {
        let split = line.split(":").collect::<Box<[_]>>();
        if let [mv, n] = split[..] {
            let k = PseudoMove::parse(mv)
                .ok_or_else(|| anyhow!("Unrecognized line of stockfish output: {} (1)", line))?;
            let v = n
                .trim()
                .parse::<usize>()
                .map_err(|_| anyhow!("Unrecognized line of stockfish output: {} (2)", line))?;
            res.insert(k, v);
        } else {
            return Err(anyhow!(
                "Unrecognized line of stockfish output: {} (3)",
                line
            ));
        }
    }

    Ok(res)
}
