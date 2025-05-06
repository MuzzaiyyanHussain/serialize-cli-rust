use borsh::{BorshSerialize, BorshDeserialize};
use std::io::{self, Write};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct User {
    username: String,
    age: u8,
}

fn main() {
    let mut username = String::new();
    let mut age_input = String::new();

    print!("Enter username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("Enter age: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut age_input).unwrap();

    let age: u8 = match age_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid number for age.");
            return;
        }
    };

    let user = User {
        username: username.trim().to_string(),
        age,
    };

    // Serialize to Borsh binary format
    let serialized = user.try_to_vec().expect("Failed to serialize");

    println!("Serialized Borsh bytes: {:?}", serialized);
}
