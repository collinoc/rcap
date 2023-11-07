use rcap::capture::{filter, read, write};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match args[..] {
        [protocol, "capture"] | [protocol, "c"] => match protocol.try_into() {
            // Todo: make a better name for this
            Ok(protocol) => write::write(protocol)?,
            Err(e) => eprintln!("{e}"),
        },
        [protocol, "read"] | [protocol, "r"] => match protocol.try_into() {
            Ok(protocol) => read::read(protocol)?,
            Err(e) => eprintln!("{e}"),
        },
        [protocol, "filter", ref extra_args @ ..] | [protocol, "f", ref extra_args @ ..] => {
            match protocol.try_into() {
                Ok(protocol) => filter::filter(protocol, extra_args)?,
                Err(e) => eprintln!("{e}"),
            }
        }
        ["--help"] | ["-h"] => {
            eprintln!("<protocol: tcp | udp | icmp | all> <mode: capture | read | filter>")
        }
        [] => {
            eprintln!("Use --help or -h for help.")
        }
        ref unk => {
            eprintln!(
                "Unknown command '{}'. Use --help or -h for help.",
                unk.join(" ")
            )
        }
    };

    Ok(())
}
