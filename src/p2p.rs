use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}, thread};

use colored::Colorize;

pub struct P2P {
    stream: TcpStream,
}

impl P2P {
    pub fn new(target: &str) -> io::Result<Self> {
        Ok(Self {
            stream: TcpStream::connect(target)?,
        })
    }

    pub fn write(&mut self, msg: &str) {
        let mut buf = msg.as_bytes();
        if let Err(why) = self.stream.write_all(&mut buf) {
            log!("ERROR: Failed to write message: {}", why);
            return;
        }

        if let Err(why) = self.stream.flush() {
            log!("ERROR: Failed to flush written tcp stream: {}", why);
            return;
        }

        log!("Sent message: {}", msg.bright_green());
    }
}

pub fn listen(port: u16) -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    log!("✅Listening for incoming messages");

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                log!("ERROR: Failed to handle connection: {}", e);
            }
        });
    }
    
    Ok(())
}


fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];

    loop {
        let bytes_read = match stream.read(&mut buf) {
            Ok(0) => {
                log!("Connection closed by target peer");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                log!("ERROR: Failed to read from stream: {}", e);
                break;
            }
        };

        let msg = String::from_utf8_lossy(&buf[..bytes_read]);
        log!("Got message: {}", msg.bright_green());
    }

    Ok(())
}