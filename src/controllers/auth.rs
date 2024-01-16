use crate::models::user::{Auth, Credentials};
use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_login::AuthSession;
use serde::Deserialize;

/////////////////////////////////////////////////////////////////////////////
//////////////////////////----- ROUTER -----/////////////////////////////////
/////////////////////////////////////////////////////////////////////////////
pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(post_login))
        .route("/login", get(login))
        .route("/logout", get(logout))
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    message: Option<String>,
    next: Option<String>,
}

// This allows us to extract the "next" field from the query string. We use this
// to redirect after log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub async fn post_login(
    mut auth_session: AuthSession<Auth>,
    Form(creds): Form<Credentials>,
) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return LoginTemplate {
                message: Some("Invalid credentials.".to_string()),
                next: creds.next,
            }
            .into_response()
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Some(ref next) = creds.next {
        Redirect::to(next).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}

pub async fn login(Query(NextUrl { next }): Query<NextUrl>) -> LoginTemplate {
    LoginTemplate {
        message: None,
        next,
    }
}

pub async fn logout(mut auth_session: AuthSession<Auth>) -> impl IntoResponse {
    match auth_session.logout().await {
        Ok(_) => Redirect::to("/login").into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
