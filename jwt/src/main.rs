use jwt::app::set_app_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = format!("0.0.0.0:{}", 8989);
    let app = set_app_router();
    let listener = TcpListener::bind(&addr).await?;
    println!("listening on: {}", addr);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
