use std::{
    collections::VecDeque,
    env::current_dir,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Stdio,
    time::Duration,
};

use crate::{
    deque,
    notation::uci::{
        Uci,
        engine::{
            self, CheckType, ComboType, EngineOption, IdString, OptionType, SpinType, StringType,
            UciEngine,
        },
        gui::{OptVal, UciGui},
    },
};
use indexmap::IndexMap;
use tokio::{
    fs::File,
    io::{
        AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter,
        Error, ErrorKind, stderr,
    },
    process::{Child, ChildStderr, ChildStdin, ChildStdout, Command},
    select,
    time::sleep,
};

use crate::profile::EngineProfile;

mod ucio;

pub async fn load_engine(prof: &EngineProfile) -> tokio::io::Result<(EngineHandle, EngineDetails)> {
    let mut handle = EngineHandle::open(
        &prof.engine.command.0,
        &prof.engine.command.1,
        prof.engine.log,
    )
    .await?;

    let mut details = EngineDetails::extract(&mut handle, Duration::from_millis(1000)).await?;

    details.load_profile(prof);

    handle.interleave(
        &mut details.set_options(),
        &mut vec![],
        Duration::from_millis(1000),
    );

    Ok((handle, details))
}

pub struct EngineDetails {
    pub name: String,
    pub author: String,
    pub options: IndexMap<String, EngineOption>,
}

impl EngineDetails {
    pub fn new(ingress: &[UciEngine]) -> Self {
        let mut res = Self {
            name: String::new(),
            author: String::new(),
            options: IndexMap::new(),
        };

        for com in ingress {
            match com {
                UciEngine::Id(IdString::Name(s)) => res.name = s.clone(),
                UciEngine::Id(IdString::Author(s)) => res.author = s.clone(),
                UciEngine::Option(opt) => {
                    res.options.insert(opt.name.clone(), opt.clone());
                }
                _ => {}
            }
        }

        res
    }

    pub async fn extract(engine: &mut EngineHandle, timeout: Duration) -> tokio::io::Result<Self> {
        let mut ingress = vec![];

        engine
            .interleave(&mut deque![UciGui::Uci()], &mut ingress, timeout)
            .await?;

        Ok(EngineDetails::new(&ingress))
    }

    pub fn load_profile(&mut self, prof: &EngineProfile) {
        use OptionType::*;
        for (key, optval) in &prof.options {
            if let Some(opt) = self.options.get_mut(key) {
                match (&mut opt.option_type, optval) {
                    (Check(ct), OptVal::Check(b)) => ct.value = Some(*b),
                    (Spin(st), OptVal::Spin(n)) => st.value = Some(*n),
                    (Combo(ct), OptVal::StringOrCombo(s)) if ct.variants.contains(s) => {
                        ct.value = Some(s.clone())
                    }
                    (String(st), OptVal::StringOrCombo(s)) => st.value = Some(s.clone()),
                    _ => {}
                }
            }
        }
    }

    pub fn set_options(&self) -> VecDeque<UciGui> {
        use OptionType::*;
        let mut res = deque![];
        for (_, opt) in &self.options {
            res.push_back(UciGui::SetOption(
                opt.name.clone(),
                match &opt.option_type {
                    Check(CheckType { value: Some(b), .. }) => OptVal::Check(*b),
                    Spin(SpinType { value: Some(n), .. }) => OptVal::Spin(*n),
                    Combo(ComboType { value: Some(s), .. }) => OptVal::StringOrCombo(s.clone()),
                    String(StringType { value: Some(s), .. }) => OptVal::StringOrCombo(s.clone()),
                    _ => continue,
                },
            ))
        }
        res
    }
}

pub struct EngineHandle {
    ingress: BufReader<ChildStdout>,
    egress: BufWriter<ChildStdin>,
    process: Child,
}

pub struct EngineEgress<'a>(&'a mut BufWriter<ChildStdin>);

pub struct EngineIngress<'a>(&'a mut BufReader<ChildStdout>);

impl EngineHandle {
    pub async fn open(
        engine: &Path,
        args: &[impl AsRef<OsStr>],
        log: bool,
    ) -> tokio::io::Result<Self> {
        let mut child = Command::new(engine);
        child
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true);

        if log {
            let mut logfile = current_dir()?;
            let engine_filename = engine
                .file_name()
                .ok_or(Error::from(ErrorKind::InvalidFilename))?;
            logfile.set_file_name(engine_filename);
            logfile.add_extension("log");

            child.stderr(File::create(logfile).await?.into_std().await);
        } else {
            child.stderr(Stdio::null());
        }

        let mut child = child.spawn()?;

        let broken_pipe = || Error::from(ErrorKind::BrokenPipe);

        let res = EngineHandle {
            ingress: BufReader::new(child.stdout.take().ok_or_else(broken_pipe)?),
            egress: BufWriter::new(child.stdin.take().ok_or_else(broken_pipe)?),
            process: child,
        };

        Ok(res)
    }

    pub async fn interleave(
        &mut self,
        egress: &mut VecDeque<UciGui>,
        ingress: &mut Vec<UciEngine>,
        timeout: Duration,
    ) -> tokio::io::Result<()> {
        let n = egress.len();
        let (mut cin, mut cout) = self.split();

        loop {
            select! {
                _ = sleep(timeout) => { break; }
                uci = cin.receive() => {
                    ingress.push(uci?);
                }
                _ = cout.send(egress.front()), if !egress.is_empty() => {
                    egress.pop_front();
                }
            }
        }

        Ok(())
    }

    pub async fn quit(&mut self) -> tokio::io::Result<()> {
        EngineEgress(&mut self.egress)
            .send(Some(&UciGui::Quit()))
            .await?;
        tokio::time::sleep(Duration::from_millis(1000)).await;
        self.egress.shutdown().await?;
        self.process.kill().await?;
        self.process.wait().await?;
        Ok(())
    }

    pub fn split<'a>(&'a mut self) -> (EngineIngress<'a>, EngineEgress<'a>) {
        (
            EngineIngress(&mut self.ingress),
            EngineEgress(&mut self.egress),
        )
    }
}

impl<'a> EngineEgress<'a> {
    #[must_use]
    pub async fn send(&mut self, uci: Option<&UciGui>) -> tokio::io::Result<bool> {
        let Some(uci) = uci else { return Ok(false) };
        let mut res = uci.to_string();
        res.push('\n');
        self.0.write_all(res.as_bytes()).await?;
        self.0.flush().await?;
        Ok(true)
    }
}

impl<'a> EngineIngress<'a> {
    #[must_use]
    pub async fn receive(&mut self) -> tokio::io::Result<UciEngine> {
        let mut buf = String::new();
        self.0.read_line(&mut buf).await?;
        Ok(UciEngine::from_string(buf))
    }
}
