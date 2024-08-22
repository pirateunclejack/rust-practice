use super::*;

use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;
// use surf::Request;
use tide::{http, Redirect, Request, Result};

static AUTH_GOOGLE_SCOPE_PROFILE: &str = "https://www.googleapis.com/auth/userinfo.profile";

#[derive(Debug, Deserialize)]
struct AuthRequestQuery {
    code: String,
    state: String,
    scope: String,
}

#[derive(Debug, Deserialize)]
struct UserInfoResponse {
    id: String,
    given_name: String,
}

pub async fn auth_google(req: Request<State>) -> Result {
    let client = &req.state().oauth_google_client;
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(AUTH_GOOGLE_SCOPE_PROFILE.to_string()))
        .url();

    Ok(Redirect::see_other(auth_url).into())
}

pub async fn auth_google_authorized(mut req: Request<State>) -> Result {
    let client = &req.state().oauth_google_client;
    let query: AuthRequestQuery = req.query()?;
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    let token_result = match token_result {
        Ok(token) => token,
        Err(_) => return Err(tide::Error::from_str(401, "error")),
    };

    let userinfo: UserInfoResponse = surf::get("https://www.googleapis.com/oauth2/v2/userinfo")
        .header(
            http::headers::AUTHORIZATION,
            format!("Bearer {}", token_result.access_token().secret()),
        )
        .recv_json()
        .await?;

    let session = req.session_mut();
    session.insert("user_name", userinfo.given_name)?;
    session.insert("user_id", userinfo.id)?;

    // println!("user_id: {}", session.get_raw("id").unwrap().to_string());
    // println!(
    //     "user_name: {}",
    //     session.get_raw("given_name").unwrap().to_string()
    // );

    Ok(Redirect::new("/").into())
}

pub async fn logout(mut req: Request<State>) -> Result {
    let session = req.session_mut();
    session.destroy();

    Ok(Redirect::new("/").into())
}
