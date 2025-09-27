use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::{
    bits::board::BitBoard,
    model::{
        Piece,
        moves::{ChessMove, PseudoMove},
    },
    zobrist::{ZobHash, ZobristBoard},
};

impl BitBoard {
    pub fn enumerate(&self, depth: usize) -> EnumerationResult {
        if depth == 0 {
            EnumerationResult {
                time: Duration::ZERO,
                depth,
                moves: HashMap::new(),
                transpos: (0, 0),
            }
        } else {
            self.clone().enumerate_mut(depth)
        }
    }

    fn enumerate_mut(&mut self, depth: usize) -> EnumerationResult {
        let now = Instant::now();
        let mut moves = HashMap::new();
        let mut zobrist = HashMap::with_capacity(10usize.pow(depth as u32));
        let hasher = ZobristBoard::new();
        let hash = hasher.hash(self);

        let mut startmvs = vec![];
        self.moves(&mut startmvs);

        let mut buf = Vec::with_capacity(startmvs.len());

        for mv in startmvs {
            let hash = hash ^ hasher.delta(mv, self.metadata.castling_details);
            self.apply(mv);
            buf.clear();
            self.moves(&mut buf);
            moves.insert(
                mv.simplify(),
                self.enum_nodes(&buf, depth - 1, hash, &mut zobrist, &hasher),
            );
            self.unapply(mv);
        }

        EnumerationResult {
            time: now.elapsed(),
            depth,
            moves,
            transpos: (zobrist.len(), zobrist.capacity()),
        }
    }

    fn enum_nodes(
        &mut self,
        moves: &[ChessMove],
        depth: usize,
        hash: ZobHash,
        zobrist: &mut HashMap<(ZobHash, usize), usize>,
        hasher: &ZobristBoard,
    ) -> usize {
        if let Some(n) = zobrist.get(&(hash, depth)) {
            return *n;
        } else if depth == 0 {
            return 1;
        } else if depth == 1 {
            let n = moves.len();
            zobrist.insert((hash, depth), n);
            return n;
        }

        let mut buf = Vec::with_capacity(moves.len());
        let mut res = 0;

        for mv in moves {
            let mv = *mv;
            let hash = hash ^ hasher.delta(mv, self.metadata.castling_details);
            let depth = depth - 1;
            if let Some(n) = zobrist.get(&(hash, depth)) {
                res += n;
            } else {
                self.apply(mv);
                buf.clear();
                self.moves(&mut buf);
                res += self.enum_nodes(&buf, depth, hash, zobrist, hasher);
                self.unapply(mv);
            }
        }

        zobrist.insert((hash, depth), res);

        return res;
    }
}

pub struct EnumerationResult {
    pub time: Duration,
    pub depth: usize,
    pub moves: HashMap<(PseudoMove, Option<Piece>), usize>,
    pub transpos: (usize, usize),
}

impl EnumerationResult {
    pub fn total(&self) -> usize {
        self.moves.values().map(|x| *x).sum()
    }

    pub fn print(&self) {
        println!("Depth searched: {}", self.depth);
        println!("Time elapsed: {} ms", self.time.as_millis());
        println!("Nodes reached: {}", self.total());
        println!("Zobrist table: {}/{}", self.transpos.0, self.transpos.1);

        for (k, v) in &self.moves {
            println!("{}: {}", k.0.longalg(k.1), v);
        }
    }
}

#[test]
fn enumerate_3() {
    let res = BitBoard::startpos().enumerate(3);

    res.print();
}
