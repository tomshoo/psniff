#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    IO(std::io::ErrorKind),
    Pcap,
    ParserError,
    General,
    Unknown,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    msg: Option<String>,
}

#[allow(dead_code)]
impl Error {
    pub fn new(kind: ErrorKind, msg: Option<String>) -> Self {
        Self { kind, msg }
    }

    fn kind(&self) -> ErrorKind {
        self.kind
    }

    fn message(&self) -> Option<String> {
        self.msg.clone()
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            kind: ErrorKind::IO(err.kind()),
            msg: None,
        }
    }
}

impl From<pcap::Error> for Error {
    fn from(err: pcap::Error) -> Self {
        Self {
            kind: ErrorKind::Pcap,
            msg: Some(err.to_string()),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Self {
            kind: ErrorKind::General,
            msg: Some(msg),
        }
    }
}

impl From<&'_ str> for Error {
    fn from(err: &'_ str) -> Self {
        Self {
            kind: ErrorKind::General,
            msg: Some(err.to_string()),
        }
    }
}

impl From<etherparse::ReadError> for Error {
    fn from(err: etherparse::ReadError) -> Self {
        if let Some(ioerr) = err.io_error() {
            Self {
                kind: ErrorKind::IO(ioerr.kind()),
                msg: None,
            }
        } else {
            Self {
                kind: ErrorKind::ParserError,
                msg: None,
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.msg {
            write!(f, "{:?}: {}", self.kind, msg)
        } else if let ErrorKind::IO(kind) = self.kind {
            write!(f, "IO: {:?}", kind)
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}
