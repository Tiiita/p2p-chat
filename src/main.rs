use std::{io::stdin, thread, time::Duration};

use colored::*;
use p2p::P2P;

macro_rules! log {
    ($($arg:tt)*) => {
        println!("{} {}", "►".bright_green(), format!($($arg)*))
    };
}

mod p2p;
fn main() {
    log!("Welcome to this p2p chat made by Tiiita!");
    println!("");
    let target = ask_for_target();
    let target_copy = target.clone();

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
    log!("Specifiy the target ip (NO PORT):");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    let ip = buf.trim().to_string();

    loop {
        log!("Specifiy the target port:");
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

fn connect_target(target: String) -> P2P {
    let mut i = 0;
    loop {
        let p2p = P2P::new(&target);
        if let Ok(_) = p2p {
            log!("✅Connected to target peer");
            return p2p.unwrap();
        }

        if i == 0 {
            log!(
                "Target machine unreachable (is it offline?): {}, {}",
                p2p.err().unwrap(),
                "trying silently".on_bright_green()
            );
        }

        thread::sleep(Duration::from_secs(1));
        i += 1;
    }
}
