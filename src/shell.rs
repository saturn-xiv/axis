use std::fmt;
use std::fs::{copy, File};
use std::io::{prelude::*, BufReader, BufWriter};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::Command as ShellCommand;

use ssh2::Session;

use super::errors::Result;

pub trait Command {
    fn script(&self, user: Option<String>, command: &str) -> Result<()>;
    fn upload<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()>;
    fn download<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()>;
}

pub enum Auth {
    Password(String),
    Key(PathBuf),
}

pub struct Ssh {
    session: Session,
    name: String,
}

impl Ssh {
    pub fn new(
        host: &str,
        port: Option<u16>,
        user: Option<String>,
        auth: Option<Auth>,
    ) -> Result<Self> {
        let user = match user {
            Some(v) => v,
            None => "root".to_string(),
        };
        let auth = match auth {
            Some(v) => v,
            None => Auth::Key(Path::new("home").join(&user).join(".ssh").join("id_sub")),
        };
        if let Auth::Key(ref file) = auth {
            if !file.exists() {
                return Err(format_err!("key file {} not exists", file.display()));
            }
        }
        let tcp = TcpStream::connect((
            host,
            match port {
                Some(p) => p,
                None => 22,
            },
        ))?;
        let peer = tcp.peer_addr()?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        match auth {
            Auth::Password(ref password) => {
                sess.userauth_password(&user, password)?;
            }
            Auth::Key(ref file) => {
                sess.userauth_pubkey_file(&user, None, file, None)?;
            }
        };
        // sess.authenticated();

        Ok(Self {
            session: sess,
            name: format!("{}@{}:{}", user, peer.ip(), peer.port()),
        })
    }
}

impl Command for Ssh {
    fn script(&self, user: Option<String>, command: &str) -> Result<()> {
        let mut channel = self.session.channel_session()?;
        channel.exec(&format!(
            "su -c \"{}\"{}",
            command,
            match user {
                Some(v) => format!(" - {}", v),
                None => "".to_string(),
            }
        ))?;

        let mut buf = String::new();
        channel.read_to_string(&mut buf)?;
        debug!("{}", buf);
        channel.wait_close()?;
        let status = channel.exit_status()?;
        if 0 == status {
            return Ok(());
        }
        Err(format_err!("shell script return {}", status))
    }
    fn upload<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        let from = File::open(from.as_ref())?;
        let mut to = self
            .session
            .scp_send(to.as_ref(), 0o400, from.metadata()?.len(), None)?;
        let mut from = BufReader::new(from);

        let mut buf = [0; 1 << 10];
        loop {
            let len = from.read(&mut buf)?;
            if len == 0 {
                break;
            }
            to.write_all(&buf[0..len])?;
        }
        Ok(())
    }
    fn download<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        let to = to.as_ref();
        let from = from.as_ref();
        let (ch, stat) = self.session.scp_recv(from)?;
        if !stat.is_file() {
            return Err(format_err!("{} isn't a file", from.display()));
        }
        let mut from = BufReader::new(ch);
        let mut to = BufWriter::new(File::create(to)?);

        let mut buf = [0; 1 << 10];
        loop {
            let len = from.read(&mut buf)?;
            if len == 0 {
                break;
            }
            to.write_all(&buf[0..len])?;
        }
        Ok(())
    }
}
impl fmt::Display for Ssh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Local;

impl Command for Local {
    fn script(&self, user: Option<String>, command: &str) -> Result<()> {
        let out = match user {
            Some(u) => ShellCommand::new("sh")
                .arg("-c")
                .arg(command)
                .arg("-")
                .arg(u)
                .output()?,
            None => ShellCommand::new("sh").arg("-c").arg(command).output()?,
        };
        debug!("{:?}", out);
        if out.status.success() {
            return Ok(());
        }
        Err(format_err!("shell script return {}", out.status))
    }
    fn upload<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        copy(from, to)?;
        Ok(())
    }
    fn download<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> Result<()> {
        copy(from, to)?;
        Ok(())
    }
}

impl fmt::Display for Local {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "localhost")
    }
}
