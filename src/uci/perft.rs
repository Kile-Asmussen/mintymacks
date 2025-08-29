use std::{
    collections::HashMap,
    hash::Hash,
    time::{Duration, Instant},
};

use crate::{
    bits::board::BitBoard,
    model::{
        Piece,
        moves::{Move, PseudoMove},
    },
};

impl BitBoard {
    pub fn perft(&self, depth: usize) -> PerfTestResult {
        if depth == 0 {
            PerfTestResult {
                time: Duration::ZERO,
                depth,
                moves: HashMap::new(),
            }
        } else {
            self.clone().perft_mut(depth)
        }
    }

    fn perft_mut(&mut self, depth: usize) -> PerfTestResult {
        let now = Instant::now();
        let mut moves = HashMap::new();

        let mut startmvs = vec![];
        self.moves(&mut startmvs);

        let mut buf = Vec::with_capacity(startmvs.len());

        for mv in startmvs {
            self.apply(mv);
            buf.clear();
            self.moves(&mut buf);
            moves.insert(mv.simplify(), self.enum_nodes(&buf, depth - 1));
            self.unapply(mv);
        }

        PerfTestResult {
            time: now.elapsed(),
            depth,
            moves,
        }
    }

    fn enum_nodes(&mut self, moves: &[Move], depth: usize) -> usize {
        if depth == 0 {
            return 1;
        } else if depth == 1 {
            return moves.len();
        }

        let mut buf = Vec::with_capacity(moves.len());
        let mut res = 0;

        for mv in moves {
            let mv = *mv;
            self.apply(mv);
            buf.clear();
            self.moves(&mut buf);
            res += self.enum_nodes(&buf, depth - 1);
            self.unapply(mv);
        }

        return res;
    }
}

pub struct PerfTestResult {
    pub time: Duration,
    pub depth: usize,
    pub moves: HashMap<(PseudoMove, Option<Piece>), usize>,
}

impl PerfTestResult {
    pub fn total(&self) -> usize {
        self.moves.values().map(|x| *x).sum()
    }

    pub fn print(&self) {
        println!("Depth searched: {}", self.depth);
        println!("Time elapsed: {} ms", self.time.as_millis());
        println!("Nodes searched: {}", self.total());

        for (k, v) in &self.moves {
            println!("{}: {}", k.0.longalg(k.1), v);
        }
    }
}

#[test]
fn perft_3() {
    let res = BitBoard::startpos().perft(3);

    res.print();
}
