use std::io::{self, prelude::*};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    println!("Client connect to server.");


    loop {
        println!("Enter a command (add_water or release_water), or 'quit' to exit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let command = input.trim();
        if command == "quit" {
            break;
        }

        stream.write_all(command.as_bytes())?;
        println!("Command sent to server; {}", command);

        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("Server response: {}", response);
    }
    Ok(())
}
