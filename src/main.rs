use clap::Parser;
use std::net::ToSocketAddrs;

#[derive(Clone, Debug)]
pub struct ResolutionQueue {
    c: usize,
    s: bool,
    p: u16,
    q: Vec<String>,
}

impl ResolutionQueue {
    pub fn new(hostnames: &[String], show_domain: bool, port: u16) -> ResolutionQueue {
        ResolutionQueue {
            c: 0,
            s: show_domain,
            p: port,
            q: hostnames.to_vec(),
        }
    }
}

impl Iterator for ResolutionQueue {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.c == self.q.len() {
            None
        } else {
            let domain = self.q[self.c].clone();
            self.c += 1;
            Some(match format!("{}:{}", &domain, self.p).to_socket_addrs() {
                Ok(addrs) => addrs
                    .map(|addr| format!("{}", if self.s {
                format!("{}\t{}", addr.ip(), domain)
            } else {
                format!("{}", addr.ip())
            }))
                    .collect::<Vec<String>>()
                    .join("\n"),
                Err(e) => {
                    format!("[failed to resolve {}: {}]", domain, e)
                }
            })
        }
    }
}

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

    #[arg(
        short,
        long,
        help = "respectively displays the domain names wherein IP-address resolve"
    )]
    pub show_domain: bool,
}

fn main() {
    let host = Terminal::parse();

    let queue = ResolutionQueue::new(&host.names, host.show_domain, host.port);

    for payload in queue {
        println!("{}", payload);
    }
}

#[cfg(test)]
mod e2e {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_resolve_ok() {
        let domains = vec![format!("github.com")];
        let resolution = ResolutionQueue::new(&domains, true, 38);
        let result = resolution.map(|n| n.clone()).collect::<Vec<String>>();
        assert_eq!(
            result,
            vec![format!(
                "{}\tgithub.com",
                "github.com:0"
                    .to_socket_addrs()
                    .unwrap()
                    .map(|g| format!("{}", g.ip()))
                    .collect::<Vec<String>>()
                    .join("\t")
            )]
        );
    }
}
