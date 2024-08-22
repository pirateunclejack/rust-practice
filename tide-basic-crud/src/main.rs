use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool};
use tera::Tera;
use tide::{http::cookies::SameSite, prelude::*, Error, Server};
use tide_tera::prelude::*;
use uuid::Uuid;

mod controllers;
mod handlers;

use controllers::{auth, dino, views};

// OAuth deps and const
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

static AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
static TOKEN_URL: &str = "https://www.googleapis.com/oauth2/v3/token";

#[derive(Clone, Debug)]
pub struct State {
    db_pool: PgPool,
    tera: Tera,
    oauth_google_client: BasicClient,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dino {
    id: Uuid,
    name: String,
    weight: i32,
    diet: String,
    user_id: Option<String>,
}

pub async fn make_db_pool(db_url: &str) -> PgPool {
    Pool::connect(db_url).await.unwrap()
}

fn make_oauth_google_client() -> tide::Result<BasicClient> {
    let client = BasicClient::new(
        ClientId::new(
            std::env::var("OAUTH_GOOGLE_CLIENT_ID").expect("OAUTH_GOOGLE_CLIENT_ID not set"),
        ),
        Some(ClientSecret::new(
            std::env::var("OAUTH_GOOGLE_CLIENT_SECRET")
                .expect("OAUTH_GOOGLE_CLIENT_SECRET not set"),
        )),
        AuthUrl::new(AUTH_URL.to_string())?,
        Some(TokenUrl::new(TOKEN_URL.to_string())?),
    )
    .set_redirect_uri(RedirectUrl::new(
        std::env::var("OAUTH_GOOGLE_REDIRECT_URL").expect("OAUTH_GOOGLE_REDIRECT_URL not set"),
    )?);

    Ok(client)
}

async fn server(db_pool: PgPool) -> Server<State> {
    let mut tera = Tera::new("templates/**/*").expect("Error parsing templates directory");
    tera.autoescape_on(vec!["html"]);

    let oauth_google_client = make_oauth_google_client().unwrap();

    let state = State {
        db_pool,
        tera,
        oauth_google_client,
    };

    let mut app = tide::with_state(state);

    app.with(
        tide::sessions::SessionMiddleware::new(
            tide::sessions::MemoryStore::new(),
            std::env::var("TIDE_SECRET")
                .expect("TIDE_SECRET not set")
                .as_bytes(),
        )
        .with_same_site_policy(SameSite::Lax),
    );

    // views

    app.at("/").get(views::index);
    app.at("/dinos/new").get(views::new);
    app.at("/dinos/:id/edit").get(views::edit);

    // auth
    app.at("/auth/google")
        .get(auth::auth_google)
        .at("/authorized")
        .get(auth::auth_google_authorized);

    app.at("/logout").get(auth::logout);

    // api
    app.at("/dinos").get(dino::list).post(dino::create);

    app.at("/dinos/:id")
        .get(dino::get)
        .put(dino::update)
        .delete(dino::delete);

    app.at("/public")
        .serve_dir("./public")
        .expect("invalid static file directory");

    app
}

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    tide::log::start();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "8888".to_string());
    let db_pool = make_db_pool(&db_url).await;

    let app = server(db_pool).await;
    let mut listener = app
        .bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Could not bind to port");

    for info in listener.info().iter() {
        println!("Listening on: {}", info);
    }
    listener.accept().await.unwrap();
}
