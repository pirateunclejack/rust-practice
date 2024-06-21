mod db;
use db::*;

fn clr() {
    print!("{}[2J", 27 as char)
}

fn main() {
    let conn = init_database().expect("Failed to connect to sqlite.");
    clr();
    let ascii = r#"
                                         _ _   
                                        | | |  
  _ __   __ _ ___ ___  __   ____ _ _   _| | |_ 
 | '_ \ / _` / __/ __| \ \ / / _` | | | | | __|
 | |_) | (_| \__ \__ \  \ V / (_| | |_| | | |_ 
 | .__/ \__,_|___/___/   \_/ \__,_|\__,_|_|\__|
 | |
 |_|
    "#;

    println!("{ascii}");
    loop {
        println!("Password manager menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clr();
                let service = &prompt("Service :");
                let username = &prompt("Username :");
                let password = &prompt("Password :");

                println!("Entry added successfully.");
                match write_password_to_db(&conn, service, username, password) {
                    Ok(()) => {}
                    Err(err) => eprintln!("Failed to save to db: {}", err),
                };
            }
            "2" => {
                clr();
                let services = match read_passwords_from_db(&conn) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("Failed to read passwords from db: {}", err);
                        Vec::new()
                    }
                };
                for item in &services {
                    println!(
                        "- Service : {}
- Username: {}
- Password: {}",
                        item.service, item.username, item.password
                    );
                    println!()
                }
            }
            "3" => {
                clr();
                let search = prompt("Search : ");
                let services = match search_service_by_name(&conn, &search) {
                    Ok(res) => res,
                    Err(err) => {
                        eprintln!("Error reading passwords:{}", err);
                        None
                    }
                };

                for item in &services {
                    if item.service.as_str() == search.as_str() {
                        println!(
                            "- Service : {}
- Username : {}
- Password : {}",
                            item.service, item.username, item.password
                        )
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice"),
        }

        println!("\n\n");
    }
}
