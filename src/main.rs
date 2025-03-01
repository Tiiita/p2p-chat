use std::{
    io::{stdin, stdout, Write},
    net::{Ipv4Addr, SocketAddrV4},
    thread,
    time::Duration,
};

use colored::*;
use igd::search_gateway;
use p2p::P2P;

macro_rules! log {
    ($($arg:tt)*) => {
        println!("{} {}", "►".bright_green(), format!($($arg)*))
    };
}

macro_rules! log_nol {
    ($($arg:tt)*) => {
        print!("{} {}", "►".bright_green(), format!($($arg)*))
    };
}

mod p2p;
fn main() {
    log!("Welcome to this p2p chat made by Tiiita!");
    println!("");
    let target = ask_for_target();
    let target_copy = target.clone();
    if let Err(why) = port_forward(target.1) {
        log!("{why}");
    }

    thread::spawn(move || {
        p2p::listen(target_copy.1).ok();
    });

    let mut p2p = connect_target(format!("{}:{}", target.0, target.1));
    loop {
        let mut in_buf = String::new();
        stdin().read_line(&mut in_buf).expect("Failed to read line");
        let input = in_buf.trim();
        p2p.write(input);
    }
}

fn ask_for_target() -> (String, u16) {
    log_nol!("Specify the target ip (NO PORT): ");
    stdout().flush().ok();
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let ip = buf.trim().to_string();

    loop {
        log_nol!("Specify the target port: ");
        stdout().flush().ok();
        let mut buf = String::new();
        stdin().read_line(&mut buf).expect("Failed to read line");
        let port = buf.trim().to_string();
        match port.parse::<u16>() {
            Ok(port) => {
                return (ip, port);
            }
            Err(_) => {
                log!("{}", "Port is invalid u16".bright_red());
            }
        }
    }
}

fn port_forward(port: u16) -> Result<(), String> {
    match search_gateway(Default::default()) {
        Ok(gateway) => {
            gateway
                .add_port(
                    igd::PortMappingProtocol::TCP,
                    port,
                    SocketAddrV4::new(Ipv4Addr::LOCALHOST, port),
                    60,
                    "p2p-chat application",
                )
                .or_else(|_| {
                    return Err("Port forwarding failed");
                })
                .ok();

            log!("✅ Port {} forwarded successfully!", port);
            Ok(())
        }
        Err(why) => {
            return Err(format!("Port forwarding not available: {}", why));
        }
    }
}

fn connect_target(target: String) -> P2P {
    let mut i = 0;
    loop {
        let p2p = P2P::new(&target);
        if let Ok(_) = p2p {
            log!("✅ Connected to target peer");
            return p2p.unwrap();
        }

        if i == 0 {
            log!("Waiting for target to connect..");
        }

        thread::sleep(Duration::from_secs(1));
        i += 1;
    }
}
