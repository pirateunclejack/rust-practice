use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter service entry: ");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read service");

        println!("Enter username entry: ");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read username");

        println!("Enter password entry: ");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read password");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string(),
        )
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
        {
            Ok(mut file) => match file.write_all(json_output.as_bytes()) {
                Ok(_) => println!("Successfully wrote to passwords.json"),
                Err(e) => eprintln!("Error writing to file: {}", e),
            },
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }
}

pub fn read_passwords_from_file() -> Result<Vec<ServiceInfo>, io::Error> {
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();
    for line in reader.lines() {
        if let Ok(json_string) = line {
            if let Ok(service_info) = ServiceInfo::from_json(&json_string) {
                services.push(service_info);
            }
        }
    }
    Ok(services)
}

pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
