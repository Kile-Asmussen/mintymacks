use std::{
    collections::{BTreeMap, HashMap, btree_map, hash_map},
    time::{Duration, Instant},
};

use rand::Rng;

use crate::{
    bits::board::BitBoard,
    fuzzing::test::{pi_rng, pi_rng_skip},
    model::{
        ChessPiece,
        moves::{ChessMove, PseudoMove},
    },
    println_async,
    utils::tree_map,
    zobrist::{ZOBRIST, ZobHash, ZobristBoard, table::ZobHashing},
};

impl BitBoard {
    pub fn enumerate(&self, depth: usize) -> EnumerationResult {
        if depth == 0 {
            EnumerationResult {
                time: Duration::ZERO,
                depth,
                moves: tree_map! {},
                transpos: (0, 0),
            }
        } else {
            self.clone().enumerate_mut(depth)
        }
    }

    fn enumerate_mut(&mut self, depth: usize) -> EnumerationResult {
        let mut depth_hashes = vec![0; depth as usize];
        pi_rng_skip(500).fill(&mut depth_hashes[..]);

        let now = Instant::now();
        let mut moves = tree_map! {};
        let mut zobrist = HashMap::with_capacity_and_hasher(10usize.pow(depth as u32), ZobHashing);
        let hash = ZOBRIST.hash(self);

        let mut startmvs = vec![];
        self.moves(&mut startmvs);

        let mut buf = Vec::with_capacity(startmvs.len());

        for mv in startmvs {
            let hash = hash ^ ZOBRIST.delta(mv, self.metadata.castling_details);
            self.apply(mv);
            buf.clear();
            self.moves(&mut buf);
            moves.insert(
                mv.simplify(),
                self.enum_nodes(&buf, depth - 1, hash, &mut zobrist, &depth_hashes[..]),
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
        zobrist: &mut HashMap<(ZobHash, ZobHash), usize, ZobHashing>,
        depths: &[ZobHash],
    ) -> usize {
        if let Some(n) = zobrist.get(&(hash, depths[depth])) {
            return *n;
        } else if depth == 0 {
            return 1;
        } else if depth == 1 {
            let n = moves.len();
            zobrist.insert((hash, depths[depth as usize]), n);
            return n;
        }

        let mut buf = Vec::with_capacity(moves.len());
        let mut res = 0;

        for mv in moves {
            let mv = *mv;
            let hash = hash ^ ZOBRIST.delta(mv, self.metadata.castling_details);
            let depth = depth - 1;
            if let Some(n) = zobrist.get(&(hash, depths[depth])) {
                res += n;
            } else {
                self.apply(mv);
                buf.clear();
                self.moves(&mut buf);
                res += self.enum_nodes(&buf, depth, hash, zobrist, depths);
                self.unapply(mv);
            }
        }

        zobrist.insert((hash, depths[depth]), res);

        return res;
    }
}

pub struct EnumerationResult {
    pub time: Duration,
    pub depth: usize,
    pub moves: BTreeMap<(PseudoMove, Option<ChessPiece>), usize>,
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

    pub async fn print_async(&self) {
        println_async!("Depth searched: {}", self.depth).await;
        println_async!("Time elapsed: {} ms", self.time.as_millis()).await;
        println_async!("Nodes reached: {}", self.total()).await;
        println_async!("Zobrist table: {}/{}", self.transpos.0, self.transpos.1).await;

        for (k, v) in &self.moves {
            println_async!("{}: {}", k.0.longalg(k.1), v).await;
        }
    }
}
