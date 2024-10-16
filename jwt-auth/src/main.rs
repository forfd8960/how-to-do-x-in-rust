use jwt_auth::{config::AppConfig, get_router, AppState};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", 8989);
    let app_config = AppConfig::new();

    let app_state = AppState::new(&app_config)?;
    let app = get_router(app_state).await?;
    let listener = TcpListener::bind(&addr).await?;
    println!("listening on: {}", addr);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
