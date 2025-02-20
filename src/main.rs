use std::fs;
use text_io::read;
use reqwest::blocking::Client;
use reqwest::StatusCode;

fn check_username(username: &str) {
    let endpoint = format!("https://forum.plutonium.pw/api/v3/users/bySlug/{}", username);
    let client = Client::new();

    match client.head(&endpoint).send() {
        Ok(response) => {
            if response.status() == StatusCode::NOT_FOUND {
                println!("[+] {}: Available", username);
            } else {
                println!("[!] {}: Unavailable", username);
            }
        }
        Err(e) => {
            eprintln!("Error checking username: {}: {}", username, e);
        }
    }
}

fn read_file(file_path: &str) {
    let usernames = fs::read_to_string(file_path)
        .expect("Should have been able to read the file!");
    
    for username in usernames.lines() {
        check_username(&username);
    }
}
    
fn main() {
    print!("Please enter the file path: ");
    let file_path: String = read!();

    read_file(&file_path);
}
