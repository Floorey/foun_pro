use std::{
    io::{self, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();
    println!("Received command from client: {}", buffer.trim());

    // Hier kannst du die empfangenen Befehle verarbeiten und entsprechend handeln
    match buffer.trim() {
        "add_water" => {
            // Führe die Logik zum Hinzufügen von Wasser aus
            println!("Processing add_water command...");
            // Beispiel: sensor3.add_water(water_amount);
        }
        "release_water" => {
            // Führe die Logik zum Freisetzen von Wasser aus
            println!("Processing release_water command...");
            // Beispiel: sensor3.release_water(sensor3.get_current_water_level());
        }
        _ => println!("Invalid command received."),
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    // Akzeptiere eingehende Verbindungen und handle sie in einem eigenen Thread
    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(move || {
            handle_client(stream);
        });
    }
    Ok(())
}
