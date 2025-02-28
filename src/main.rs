use colored::Colorize;

fn main() {
    log("Welcome to the p2p chat made by Tiiita!");
    
}


pub fn log(msg: &str) {
    let prefix = "►".bright_green();
    println!("{prefix} {msg}");
}