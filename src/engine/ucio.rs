use std::time::Duration;

use tokio::{
    io::{
        AsyncBufRead, AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, Stdin, Stdout, stdin,
        stdout,
    },
    select,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::sleep,
};
use tokio_util::sync::CancellationToken;

use crate::{
    deque,
    engine::{EngineDetails, ucio},
    notation::uci::{
        engine::{IdString, InfoString, UciEngine},
        gui::UciGui,
    },
};

pub struct GoStopInfo {
    receiver: UnboundedReceiver<Vec<InfoString>>,
    sender: UnboundedSender<Vec<InfoString>>,
    stop: tokio_util::sync::CancellationToken,
    _drop: tokio_util::sync::DropGuard,
}

impl GoStopInfo {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let stop = CancellationToken::new();
        let _drop = stop.clone().drop_guard();
        GoStopInfo {
            receiver,
            sender,
            stop,
            _drop,
        }
    }

    pub fn ucout(&self) -> Infout {
        Infout {
            sender: self.sender.clone(),
            stop: self.stop.clone(),
        }
    }

    pub fn react(&mut self, uci: &UciGui) -> bool {
        match uci {
            UciGui::Go(_) | UciGui::PonderHit() | UciGui::UciNewGame() => {
                self.stop.cancel();
                self.receiver.close();
                self.stop = CancellationToken::new();
                self._drop = self.stop.clone().drop_guard();
                true
            }
            UciGui::Quit() => {
                self.stop.cancel();
                self.receiver.close();
                true
            }
            _ => false,
        }
    }

    pub async fn listen(&mut self, timeout: Duration) -> tokio::io::Result<()> {
        let mut ucin = Ucin::new();
        let mut ucout = Ucout::new();
        let mut outbox = deque![];

        loop {
            select! {
                _ = sleep(timeout) => break,
                _ = ucout.send(outbox.front()), if !outbox.is_empty() => {
                    outbox.pop_front();
                }
                uci = ucin.receive() => {
                    if let Ok(uci) = uci? {
                        self.react(&uci);
                    }
                }
                uci = self.receiver.recv() => {
                    if let Some(uci) = uci {
                        outbox.push_back(UciEngine::Info(uci));
                    }
                }
            }
        }

        Ok(())
    }
}

pub struct Ucin {
    reader: BufReader<Stdin>,
}

impl Ucin {
    pub fn new() -> Self {
        Self {
            reader: BufReader::new(stdin()),
        }
    }

    pub async fn receive(&mut self) -> tokio::io::Result<Result<UciGui, String>> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf).await?;
        Ok(UciGui::from_str(buf))
    }
}

pub struct Ucout {
    writer: BufWriter<Stdout>,
}

impl Ucout {
    pub fn new() -> Self {
        Self {
            writer: BufWriter::new(stdout()),
        }
    }

    pub async fn send(&mut self, uci: Option<&UciEngine>) -> tokio::io::Result<()> {
        if let Some(uci) = uci {
            self.writer.write_all(uci.to_string().as_bytes()).await?
        }
        Ok(())
    }
}

pub struct Infout {
    sender: UnboundedSender<Vec<InfoString>>,
    stop: CancellationToken,
}

impl EngineDetails {
    pub fn dump(&self) -> Vec<UciEngine> {
        let mut res = vec![];

        res.push(UciEngine::Id(IdString::Name(self.name.clone())));
        res.push(UciEngine::Id(IdString::Author(self.author.clone())));

        for opt in self.options.values() {
            res.push(UciEngine::Option(opt.clone()))
        }

        res
    }
}
