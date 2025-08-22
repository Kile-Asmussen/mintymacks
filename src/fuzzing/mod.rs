use std::{collections::HashMap, process::Command};

use anyhow::anyhow;

use crate::{
    model::{
        Piece,
        moves::{Move, PseudoMove},
    },
    uci::longalg::parse_long_alg,
};

pub fn stockfish_perft(
    moves: &[Move],
    depth: usize,
) -> anyhow::Result<HashMap<(PseudoMove, Option<Piece>), usize>> {
    let mut res = HashMap::new();

    let output = String::from_utf8(
        Command::new("./src/fuzzing/stockfish-perft.sh")
            .arg(
                moves
                    .into_iter()
                    .map(|m| m.to_longalg())
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
            let k = parse_long_alg(mv)
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
