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
    let mut p2p = connect_target(target);

    thread::spawn(move || {
        p2p::listen(&target_copy).ok();
    });

    loop {
        let mut in_buf = String::new();
        stdin().read_line(&mut in_buf).expect("Failed to read line");
        let input = in_buf.trim();
        p2p.write(input);
    }
}

fn ask_for_target() -> String {
    log!("Specifiy the target address:");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read line");
    buf.trim().to_string()
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
            log!("Target machine unreachable (is it offline?): {}, {}", p2p.err().unwrap(), "trying silently..".on_bright_green());
        }

        thread::sleep(Duration::from_secs(1));
        i+=1;
    }
}