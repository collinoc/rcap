use super::{Protocol, BUF_SIZE};
use nix::sys::socket::{self, socket, AddressFamily, MsgFlags, SockFlag, SockProtocol, SockType};
use std::{
    io::{self, Write},
    os::fd::AsRawFd,
};

pub fn write(protocol: Protocol) -> io::Result<()> {
    let domain = match protocol {
        Protocol::All => AddressFamily::Packet,
        Protocol::Tcp | Protocol::Udp | Protocol::Icmp => AddressFamily::Inet,
    };

    let ty = match protocol {
        Protocol::All => SockType::Raw,
        Protocol::Tcp => SockType::Stream,
        Protocol::Udp => SockType::Datagram,
        Protocol::Icmp => SockType::Raw, // ??
    };

    let flags = SockFlag::empty();

    let protocol = match protocol {
        Protocol::All => SockProtocol::EthAll,
        Protocol::Tcp => SockProtocol::Tcp,
        Protocol::Udp => SockProtocol::Udp,
        // TODO: SockProtocol::Icmp is not yet publicly available.
        // A fix for this was merged recently. CanRaw can be used in the meantime.
        Protocol::Icmp => SockProtocol::CanRaw,
    };

    let sock = socket(domain, ty, flags, protocol).expect("failed to create socket");
    let sock_raw = sock.as_raw_fd();

    let mut stdout = io::stdout().lock();
    let mut buffer = [0u8; BUF_SIZE];
    let msg_flags = MsgFlags::empty(); // Look into MSG_TRUNC

    loop {
        let read: usize = socket::recv(sock_raw, &mut buffer, msg_flags)?;
        let out = &buffer[..read];

        eprintln!("Read: {read}");

        stdout.write_all(out)?;
        stdout.flush()?;
    }
}
