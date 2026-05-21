use std::net::SocketAddr;
use axum::{routing::get, Router};
use dotenvy::dotenv;

mod models;

#[tokio::main]
async fn main() {
    // Cargar variables de entorno desde el archivo .env si existe
    dotenv().ok();

    println!("==================================================");
    println!("🚀 Iniciando servidor backend de CL_Miscelanea_POS");
    println!("==================================================");

    // Inicializar router base de Axum
    let app = Router::new()
        .route("/health", get(health_check));

    // Definir dirección del servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("📡 Servidor escuchando en: http://{}", addr);

    // Iniciar servidor
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "Backend OK - Tabla 'empresas' lista para usar!"
}
