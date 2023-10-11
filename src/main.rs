use clap::Parser;
use std::net::ToSocketAddrs;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Terminal {
    #[arg(value_name = "HOSTNAMES", required = true)]
    pub names: Vec<String>,

    #[arg(short, long, value_name = "PORT", default_value_t = 80)]
    pub port: u16,

    #[arg(short, long, help = "crash upon non-ability to name-resolution")]
    pub crash: bool,

    #[arg(short, long, help = "respectively displays the domain names wherein IP-address resolve")]
    pub show_domain: bool,
}

fn main() {
    let host = Terminal::parse();

    for domain in host.names.iter() {
        match format!("{}:{}", &domain, host.port).to_socket_addrs() {
            Ok(addrs) => {
                for addr in addrs {
                    if host.show_domain {
                        println!("{}\t{}", addr.ip(), domain);
                    } else {
                        println!("{}", addr.ip());
                    }
                }
            },
            Err(e) => {
                eprintln!("could not resolve {:#?}: {}", domain, e);
                std::process::exit(0x31);
            }
        }
    }
}
