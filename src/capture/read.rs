use super::Protocol;
use std::io::{self, Read};

pub fn read(_protocol: Protocol) -> io::Result<()> {
    let mut buffer = [0u8; super::BUF_SIZE];
    let mut stdin = io::stdin().lock();

    loop {
        let read = stdin.read(&mut buffer)?;

        if read == 0 {
            break;
        }

        println!("Read {read} bytes");

        // todo: parse bytes

        buffer.fill(0);
    }

    Ok(())
}
