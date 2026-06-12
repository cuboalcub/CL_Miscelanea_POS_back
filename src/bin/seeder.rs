use std::path::PathBuf;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("SEEDER_DATABASE_URL")
        .or_else(|_| std::env::var("DATABASE_URL"))
        .expect("SEEDER_DATABASE_URL o DATABASE_URL debe estar configurada");

    println!("==============================================");
    println!("🌱 Seeder de Catálogos SAT para CFDI 4.0");
    println!("==============================================");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    let seed_path = std::env::var("SEED_PATH")
        .unwrap_or_else(|_| "seeds/sat_catalogos.sql".to_string());

    let path = PathBuf::from(&seed_path);
    let sql = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("No se pudo leer el archivo '{}': {}", path.display(), e));

    println!("📄 Ejecutando seed desde: {}", path.display());
    println!("⚙️  Procesando catálogos...");

    let start = std::time::Instant::now();

    sqlx::query(&sql)
        .execute(&pool)
        .await
        .expect("Error al ejecutar el script de seed");

    let elapsed = start.elapsed();

    println!("✅ Catálogos SAT sembrados exitosamente en {:.2?}s", elapsed);
    println!("");
    println!("📋 Catálogos incluidos:");
    println!("   1. c_FormaPago      — Formas de pago");
    println!("   2. c_MetodoPago     — Métodos de pago");
    println!("   3. c_UsoCFDI        — Usos del CFDI");
    println!("   4. c_RegimenFiscal  — Regímenes fiscales");
    println!("   5. c_TipoComprobante — Tipos de comprobante");
    println!("   6. c_Exportacion    — Claves de exportación");
    println!("   7. c_ClaveProdServ  — Claves de productos/servicios");
    println!("   8. c_ClaveUnidad    — Unidades de medida");
    println!("");
    println!("💡 Ejecuta de nuevo sin riesgo (idempotente).");
    println!("");
    println!("🔑 Usa SEEDER_DATABASE_URL (superuser) si DATABASE_URL usa app_user con RLS.");
}
