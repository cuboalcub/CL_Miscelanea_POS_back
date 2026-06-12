#![allow(dead_code)]

use std::net::SocketAddr;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::broadcast;

mod auth;
mod errors;
mod handlers;
mod middleware;
mod models;
mod processors;
mod realtime;
mod repositories;
mod router;
mod state;

use realtime::StockAlertEvent;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("La variable de entorno DATABASE_URL debe estar configurada");

    println!("==================================================");
    println!("🚀 Iniciando servidor backend de CL_Miscelanea_POS");
    println!("==================================================");

    println!("🔌 Conectando a la base de datos...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");
    println!("✅ Conexión establecida con éxito.");

    println!("⚙️ Corriendo migraciones pendientes...");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Fallo al correr las migraciones");
    println!("✅ Migraciones aplicadas con éxito.");

    let (stock_tx, _) = broadcast::channel::<StockAlertEvent>(256);

    let state = AppState::new(pool, stock_tx);
    let app = router::build(state);

    let _host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(8080);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("📡 Servidor escuchando en: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
