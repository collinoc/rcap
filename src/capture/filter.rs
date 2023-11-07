use super::Protocol;
use std::io;

pub fn filter(_protocol: Protocol, _args: &[&str]) -> io::Result<()> {
    Ok(())
}
