use std::io::{self, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    // Sende Befehle an den Server
    let commands = vec!["add_water", "release_water"];

    for command in commands {
        stream.write_all(command.as_bytes())?;
        println!("Command sent to server: {}", command);

        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("Server response: {}", response);
    }

    Ok(())
}
