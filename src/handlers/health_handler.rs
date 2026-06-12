/// Handler de salud del servidor. Responsabilidad única: responder al health check.
pub async fn health_check() -> &'static str {
    "Backend OK - Servidor y base de datos activos!"
}
