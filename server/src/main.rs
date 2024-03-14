struct Message {
    sender: String,
    content: String,
}

struct User {
    username: String,
}

struct Room {
    name: String,
    users: Vec<User>,
}

// Functions to handle encryption and decryption
fn encrypt_message(message: &str, key: &[u8]) -> Vec<u8> {
    let encrypted_data: Vec<u8> = Vec::new(); // Placeholder, replace with actual encryption
    encrypted_data
}

fn decrypt_message(encrypted_message: &[u8], key: &[u8]) -> String {
    let decrypted_data: String = String::from_utf8_lossy(encrypted_message).to_string(); // Placeholder, replace with actual decryption
    decrypted_data
}

// Function for communication with the server
fn receive_message() -> Message {
    // Placeholder implementation for demonstration purposes
    // Replace this with your actual message receiving logic
    let received_message = Message {
        sender: String::from("Alice"),
        content: String::from("Hello from Alice!"),
    };
    received_message
}

// Functions for communication with the server
fn connect_to_server(ip: &str) {
    // Connect to the server
}

fn join_room(room_name: &str) {
    // Join a specific room
}

fn send_message(room_name: &str, message: &str) {
    // Encrypt the message
    // Send the encrypted message to the server
}

// Main function
fn main() {
    // Connect to the server
    connect_to_server("server_ip");

    // Join a room
    join_room("room_name");

    // Loop to send and receive messages
    // this comment is for my kernel friend who is very nice person
    loop {
        // Send a message
        let message = "";
        send_message("room_name", message);

        // Receive a message
        let received_message = receive_message();
        println!(
            "Received: {} from {}",
            received_message.content, received_message.sender
        );
    }
}

// i think about quitting rust fck this
