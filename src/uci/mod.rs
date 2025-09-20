use std::collections::VecDeque;
use std::io::{Error, ErrorKind, Read, StdinLock, StdoutLock, Write, stdin, stdout};
use std::process::Child;

use mio::Poll;
use mio::unix::SourceFd;
use mio::unix::pipe::{Receiver, Sender};
use nix::errno::Errno;
use nix::fcntl::{self, FcntlArg, OFlag, fcntl};

pub struct IAmAnEngine {
    outbox: VecDeque<String>,
    inbox: VecDeque<String>,
    stdin: StdinLock<'static>,
    stdout: StdoutLock<'static>,
}

impl AmEngine {
    pub fn new() -> Result<Self, Errno> {
        let stdin = stdin().lock();
        let stdout = stdout().lock();
        let f = fcntl(&stdin, FcntlArg::F_GETFL)?;
        fcntl(&stdin, FcntlArg::F_SETFL(OFlag::O_NONBLOCK))?;

        Ok(Self {
            outbox: vec![],
            inbox: vec![],
            stdin,
            stdout,
        })
    }
}

pub struct Engine {
    outbox: VecDeque<String>,
    inbox: VecDeque<String>,
    sender: Sender,
    receiver: Receiver,
}

impl Engine {
    pub fn new(c: &mut Child) -> Option<Self> {
        Some(Self {
            outbox: vec![],
            inbox: vec![],
            sender: Sender::from(c.stdin.take()?),
            receiver: Receiver::from(c.stdout.take()?),
        })
    }
}

trait Communicator {
    fn push(&mut self, s: String);
    fn try_pull(&mut self) -> Option<String>;
    fn poll_out(&mut self) -> Result<bool, std::io::Error>;
    fn poll_in(&mut self) -> Result<bool, std::io::Error>;
}

impl Communicator for Engine {
    fn poll_out(&mut self) -> Result<bool, std::io::Error> {
        let mut n = 0;
        while let Some(s) = self.outbox.pop_back() {
            match self.sender.write_fmt(format_args!("{s}")) {
                Ok(()) => n += 1,
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        self.outbox.push_back(s);
                        return Ok(n != 0);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(false)
    }

    fn poll_in(&mut self) -> Result<bool, std::io::Error> {
        let mut n = 0;
        loop {
            let mut buf = String::new();
            match self.receiver.read_to_string(&mut buf) {
                Ok(0) => return Ok(n != 0),
                Ok(1..) => self.inbox.extend(buf.split("\n").map(ToString::to_string)),
                Err(e) if e.kind() == ErrorKind::WouldBlock => return Ok(n != 0),
                Err(e) => return Err(e),
            }
        }
    }

    fn push(&mut self, s: String) {
        self.outbox.push_front(s);
    }

    fn try_pull(&mut self) -> Option<String> {
        self.inbox.pop_back()
    }
}
