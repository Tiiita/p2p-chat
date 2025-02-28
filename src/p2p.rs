use std::{io::{self, Read, Write}, net::{TcpListener, TcpStream}};

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
        }

        if let Err(why) = self.stream.flush() {
            log!("ERROR: Failed to flush written tcp stream: {}", why);
        }

        log!("Sent message: {}", msg.bright_green());
    }
}

pub fn listen(target: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(target)?;
    log!("✅Listening for incoming messages");

    for stream in listener.incoming() {
        let stream = stream?;
        if let Err(e) = handle_connection(stream) {
            log!("ERROR: Failed to handle connection: {}", e);
        }
    }
    
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    stream.read(&mut buf)?;

    let msg = String::from_utf8(buf.to_vec())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    
    log!("Got message: {}", msg.bright_green());
    Ok(())
}