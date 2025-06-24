use std::io;
use std::{error::Error, io::Write};

use crate::{
    models::{Movie, Role},
    services::{
        get_logged_in_role, get_users, list_movies, login_success, logout, read_from_json,
        write_to_json,
    },
};

pub fn handle_login(username: &str) -> Result<(), Box<dyn Error>> {
    println!("{}", username);
    if let Some(user) = get_users()
        .iter()
        .find(|u| u.username.eq_ignore_ascii_case(username))
    {
        println!("Please enter the password: ");
        match rpassword::read_password() {
            Ok(password) => {
                if user.password == password {
                    login_success(&user.role)?;
                    println!("Log in successfully.");
                } else {
                    println!("Incorrect password.");
                }
            }
            Err(_) => {
                println!("Failed to read password")
            }
        }
        // let mut pw = String::new();
        // if io::stdin().read_line(&mut pw).is_ok() {
        //     println!("Log in successfully");
        // } else {
        //     println!("Failed to read password")
        // }
    } else {
        println!("User not found")
    }
    Ok(())
}

pub fn handle_logout() {
    logout();
    println!("Logged out successfully.")
}

pub fn handle_list() -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(_) => {
            let movies = read_from_json()?;
            list_movies(&movies);
            // println!("{movies:#?}");
        }
        None => {
            println!("You need to log in to view movies.")
        }
    }
    Ok(())
}

pub fn handle_add(
    disc: usize,
    year: &str,
    title: &str,
    remark: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    match get_logged_in_role()? {
        Some(Role::Admin) => {
            let mut movies = read_from_json()?;
            let new_movie = Movie {
                disc,
                year: year.to_string(),
                title: title.to_string(),
                remark: remark.clone(),
            };
            movies.push(new_movie);
            write_to_json(&movies)?;
            println!("Movie added.")
        }
        _ => {
            println!("You need to log in as Admin to add movie");
        }
    }
    Ok(())
}

pub fn handle_delete(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()? {
        let movies = read_from_json()?;
        if let Some(movie) = movies
            .iter()
            .filter(|m| m.disc == *disc)
            .enumerate()
            .find(|(i, _)| i == index)
            .map(|(_, m)| m.clone())
        {
            let left_movies = movies
                .into_iter()
                .filter(|m| *m != movie)
                .collect::<Vec<Movie>>();

            write_to_json(&left_movies)?;
            println!("Movie deleted.");
        }
    }
    Ok(())
}

pub fn handle_edit(disc: &usize, index: &usize) -> Result<(), Box<dyn Error>> {
    if let Some(Role::Admin) = get_logged_in_role()? {
        let mut movies = read_from_json()?;
        if let Some(movie) = movies
            .iter_mut()
            .filter(|m| m.disc == *disc)
            .enumerate()
            .find(|(i, _)| i == index)
            .map(|(_, m)| m)
        {
            println!("Enter the new disc no.: ");
            io::stdout().flush()?;
            let mut disc = String::new();
            io::stdin().read_line(&mut disc)?;
            let disc = disc.trim();
            if let Ok(disc) = disc.parse::<usize>() {
                movie.disc = disc;
            } else {
                println!("Invalid disc number.");
                return Ok(());
            }

            println!("Enter the new year: ");
            io::stdout().flush()?;
            let mut year = String::new();
            io::stdin().read_line(&mut year)?;
            let year = year.trim();
            movie.year = year.to_string();

            println!("Enter the new title: ");
            io::stdout().flush()?;
            let mut title = String::new();
            io::stdin().read_line(&mut title)?;
            let title = title.trim();
            movie.title = title.to_string();

            println!("Enter the new remark (optional): ");
            io::stdout().flush()?;
            let mut remark = String::new();
            io::stdin().read_line(&mut remark)?;
            let remark = remark.trim();

            if title.is_empty() {
                movie.remark = None;
            } else {
                movie.remark = remark.to_string().into();
            }

            write_to_json(&movies)?;

            println!("Movie modified.");
        }
    } else {
        println!("You need to log in as admin to edit a movie.");
    }
    Ok(())
}
