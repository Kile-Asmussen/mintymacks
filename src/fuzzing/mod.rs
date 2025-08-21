use crate::bits::board::BitBoard;
use crate::model::Piece;
use crate::model::moves::{Move, PseudoMove};
use crate::uci::longalg::parse_long_alg;
use anyhow::anyhow;
use rand::Rng;
use rand::seq::IndexedRandom;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::Duration;

pub fn run_stockfish<R: Rng>(r: &mut R) -> anyhow::Result<()> {
    let mut sf = Stockfish::new()?;

    let mut board = BitBoard::startpos();
    let mut moves = vec![];
    let mut history = vec![];

    for i in 0..50 {
        moves.clear();
        board.moves(&mut moves);
        let Some(move_to_make) = moves.choose(r).map(|m| *m) else {
            break;
        };

        let pmoves = sf.check_position(&history)?;

        for (mv, prom) in pmoves {
            let Some(pos) = moves.iter().position(|m| m.matches(mv, prom)) else {
                return crash_out(&[(mv, prom)], &[], &board);
            };
            moves.swap_remove(pos);
        }

        if moves.len() != 0 {
            return crash_out(&[], &moves, &board);
        }

        board.apply(move_to_make);
        history.push(move_to_make);
    }

    return Ok(());

    fn crash_out(
        not_found: &[(PseudoMove, Option<Piece>)],
        left_over: &[Move],
        board: &BitBoard,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

pub struct Stockfish {
    process: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl Stockfish {
    pub fn new() -> anyhow::Result<Stockfish> {
        let mut process = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let stdin = process.stdin.take().ok_or(anyhow!("No stdin"))?;
        let stdout = process.stdout.take().ok_or(anyhow!("No stdout"))?;

        Ok(Stockfish {
            process,
            stdin,
            stdout,
        })
    }

    pub fn startpos_moves(&mut self, moves: &[Move]) -> anyhow::Result<()> {
        self.talk(&format!(
            "position startpos move {}",
            moves
                .into_iter()
                .map(|m| m.to_uci())
                .collect::<Vec<_>>()
                .join(" ")
        ))?;
        Ok(())
    }

    pub fn go_perft_1(&mut self) -> anyhow::Result<String> {
        self.talk("go perft 1")?;
        thread::sleep(Duration::from_millis(50));
        Ok(self.listen()?)
    }

    pub fn check_position(
        &mut self,
        moves: &[Move],
    ) -> anyhow::Result<Vec<(PseudoMove, Option<Piece>)>> {
        let mut res = vec![];
        self.startpos_moves(moves)?;
        let printout = self.go_perft_1()?;
        for line in printout.lines() {
            if line.ends_with(": 1") {
                let mv = parse_long_alg(&line[..line.find(":").unwrap()])
                    .ok_or_else(|| anyhow!("Unrecognized long algebraic form {}", line))?;
                res.push(mv);
            }
        }
        Ok(res)
    }

    pub fn listen(&mut self) -> anyhow::Result<String> {
        let mut res = String::new();
        let mut buf = [0u8; 1024];
        loop {
            let n = self.stdout.read(&mut buf[..])?;
            res += str::from_utf8(&buf[..n])?;
            if n < buf.len() {
                break;
            }
        }
        Ok(res)
    }

    pub fn talk(&mut self, say: &str) -> anyhow::Result<()> {
        self.stdin.write(say.as_bytes())?;
        self.stdin.as_fd();
        self.stdin.flush()?;
        Ok(())
    }

    pub fn done(&mut self) -> anyhow::Result<()> {
        self.process.kill()?;
        Ok(())
    }
}
