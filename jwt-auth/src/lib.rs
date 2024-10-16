use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use blog::Blog;
use config::AppConfig;
use errors::AppError;
use jwt::{DecodingKey, EncodingKey};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlog {
    title: String,
    content: String,
}

pub async fn get_router(state: AppState) -> Result<Router, AppError> {
    let api = Router::new()
        .route("/blog", post(create_post))
        .route("/signin", post(signin_handler))
        .with_state(state);
    // .layer(from_fn_with_state(state.clone(), verify_token::<AppState>));

    Ok(api)
}

#[axum::debug_handler]
async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SignInUser>,
) -> Result<impl IntoResponse, AppError> {
    Ok((
        StatusCode::OK,
        Json(User::new(input.username, input.password)),
    )
        .into_response())
}

#[axum::debug_handler]
async fn create_post(
    State(_state): State<AppState>,
    Json(create_blog): Json<CreateBlog>,
) -> Result<impl IntoResponse, AppError> {
    let blog = Blog::new(create_blog.title, create_blog.content);
    Ok((StatusCode::OK, Json(blog)).into_response())
}
