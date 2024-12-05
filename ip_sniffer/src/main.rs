use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use tokio::net::TcpStream;
use tokio::task;

const MAX: u16 = 65535;
const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(172, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
// Address argument.  Accepts -a and --address and an IpAddr type. Falls back to the above constant.
struct Arguments {
    #[bpaf(long, short, argument("address"), fallback(IP_FALLBACK))]
    /// The address that you want to sniff.  Must be a valid ipv4 address.  Falls back to 127.0.0.1
    address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        argument("start_port"),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    /// The start port for sniffer. (must be greater than 0)
    start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        argument("end_port"),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    /// The end port for the sniffer. (must be less than or equal to 65535)
    end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input < MAX
}

async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(start_port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts = arguments().run();
    let (tx, rx) = channel();
    for port in opts.start_port..opts.end_port {
        let tx = tx.clone();

        task::spawn(async move { scan(tx, port, opts.address).await });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
