pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    All,
}

impl TryFrom<&str> for Protocol {
    type Error = error::CaptureError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "tcp" => Ok(Protocol::Tcp),
            "udp" => Ok(Protocol::Udp),
            "icmp" => Ok(Protocol::Icmp),
            "all" => Ok(Protocol::All),
            p => Err(Self::Error::UnknownProtocol(p.to_string())),
        }
    }
}

pub const BUF_SIZE: usize = 4096;

pub mod error;
pub mod filter;
pub mod read;
pub mod write;
