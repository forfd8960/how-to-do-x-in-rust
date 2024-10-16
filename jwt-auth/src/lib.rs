use axum::{
    extract::State, http::StatusCode, middleware::from_fn_with_state, response::IntoResponse,
    routing::post, Extension, Json, Router,
};
use blog::Blog;
use config::AppConfig;
use errors::AppError;
use jwt::{DecodingKey, EncodingKey};
use middleware::verify_token;
use serde::{Deserialize, Serialize};
use std::{ops::Deref, sync::Arc};
use user::User;

pub mod auth;
pub mod blog;
pub mod config;
pub mod errors;
pub mod jwt;
pub mod middleware;
pub mod user;

#[derive(Clone)]
pub struct AppState {
    state: Arc<AppStateInner>,
}

#[derive(Clone)]
pub struct AppStateInner {
    pk: EncodingKey,
    dk: DecodingKey,
}

impl AppState {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        let pk = EncodingKey::load(&config.private_pem)?;
        let dk = DecodingKey::load(&config.public_pem)?;

        Ok(Self {
            state: Arc::new(AppStateInner { pk, dk }),
        })
    }
}

impl Deref for AppState {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInUser {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct SignInResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlog {
    title: String,
    content: String,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api = Router::new()
        .route("/blog", post(create_post))
        .layer(from_fn_with_state(state.clone(), verify_token::<AppState>))
        .route("/signin", post(signin_handler))
        .with_state(state);

    Ok(api)
}

#[axum::debug_handler]
async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SignInUser>,
) -> Result<impl IntoResponse, AppError> {
    if input.username.is_empty() {
        return Ok((StatusCode::BAD_REQUEST, "invalid user name").into_response());
    }

    if input.password.len() < 10 {
        return Ok((StatusCode::BAD_REQUEST, "too short password").into_response());
    }

    let user = User::new(input.username, input.password);
    let token = state.pk.sign(user)?;
    Ok((StatusCode::OK, Json(SignInResponse { token })).into_response())
}

#[axum::debug_handler]
async fn create_post(
    Extension(user): Extension<User>,
    State(_state): State<AppState>,
    Json(create_blog): Json<CreateBlog>,
) -> Result<impl IntoResponse, AppError> {
    let blog = Blog::new(user, create_blog.title, create_blog.content);
    Ok((StatusCode::OK, Json(blog)).into_response())
}
