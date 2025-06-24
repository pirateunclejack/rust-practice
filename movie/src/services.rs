use std::error::Error;
use std::fs;
use std::io;

use unicode_width::UnicodeWidthStr;

use crate::models::Movie;
use crate::models::Role;
use crate::models::User;

pub fn get_users() -> Vec<User> {
    vec![
        User {
            username: "Admin".to_string(),
            password: "password".to_string(),
            role: Role::Admin,
        },
        User {
            username: "Dave".to_string(),
            password: "Mustaine".to_string(),
            role: Role::User,
        },
        User {
            username: "Nick".to_string(),
            password: "Carter".to_string(),
            role: Role::User,
        },
    ]
}

pub fn login_success(role: &Role) -> Result<(), Box<dyn Error>> {
    fs::write(".session", role.to_string())?;
    Ok(())
}

pub fn get_logged_in_role() -> Result<Option<Role>, Box<dyn Error>> {
    let role = fs::read_to_string(".session")?;
    match role.as_str() {
        "Administrator" => Ok(Some(Role::Admin)),
        "User" => Ok(Some(Role::User)),
        _ => Ok(None),
    }
}

pub fn logout() {
    fs::remove_file(".session").unwrap_or_else(|_| {
        println!("No user is logged in.");
    })
}

pub fn read_from_json() -> Result<Vec<Movie>, Box<dyn Error>> {
    let file = fs::File::open("movies.json")?;
    let reader = std::io::BufReader::new(file);
    let movies: Vec<Movie> = serde_json::from_reader(reader)?;
    Ok(movies)
}

pub fn list_movies(movies: &[Movie]) {
    println!("{:<5}{:<7}{:<80}{:<15}", "Disc", "Year", "Title", "Remark");
    println!("{:-<110}", "");
    movies.iter().for_each(|m| {
        let remark = m.remark.as_deref().unwrap_or("");
        let title = pad_display_width(&m.title, 80);
        let remark = pad_display_width(remark, 15);
        println!("{:<5}{:<7}{}{}", m.disc, m.year, title, remark);
    });
}

fn pad_display_width(s: &str, target_width: usize) -> String {
    let width = UnicodeWidthStr::width(s);
    format!("{}{}", s, " ".repeat(target_width.saturating_sub(width)))
}

pub fn write_to_json(movies: &[Movie]) -> Result<(), Box<dyn Error>> {
    let file = fs::File::create("movies.json")?;
    let writer = io::BufWriter::new(file);
    serde_json::to_writer(writer, movies)?;
    Ok(())
}
