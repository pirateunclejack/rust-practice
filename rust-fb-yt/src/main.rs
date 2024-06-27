use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}

// convert a string to a response
fn string_to_response(s: &str) -> Response {
    println!("Response string: {:?}", s);
    serde_json::from_str(s).unwrap()
}

// convert a string to a user
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let _users = firebase.set::<User>(&user).await;
    println!("set_user response: {:?}", _users);
    return string_to_response(&_users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;
    println!("All users: {:?}", users);
    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;
    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

#[tokio::main]
async fn main() {
    let user = User {
        name: "name1".to_string(),
        age: 30,
        email: "name1@gmail.com".to_string(),
    };

    let firebase: Firebase = Firebase::new("********************************").unwrap();

    let response = set_user(&firebase, &user).await;
    println!("Response {:?}", response);

    let mut user = get_user(&firebase, &response.name).await;
    println!("User: {:?}", user);

    let users = get_users(&firebase).await;
    println!("All users: {:?}", users);

    user.email = "newname1@gmail.com".to_string();
    let updated_user = update_user(&firebase, &response.name, &user).await;
    println!("Updated user: {:?}", updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted: {:?}", &response.name)
}
