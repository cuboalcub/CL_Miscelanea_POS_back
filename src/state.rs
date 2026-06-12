use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::realtime::StockAlertEvent;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub admin_db: PgPool,
    pub stock_tx: broadcast::Sender<StockAlertEvent>,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
}

impl AppState {
    pub fn new(db: PgPool, admin_db: PgPool, stock_tx: broadcast::Sender<StockAlertEvent>) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "cl_miscelanea_pos_jwt_secret_default".to_string());
        let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24);

        Self { db, admin_db, stock_tx, jwt_secret, jwt_expiration_hours }
    }

    pub async fn set_tenant_context(&self, empresa_id: Uuid, usuario_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT set_config('app.current_empresa_id', $1, true)")
            .bind(empresa_id.to_string())
            .execute(&self.db)
            .await?;
        sqlx::query("SELECT set_config('app.current_usuario_id', $1, true)")
            .bind(usuario_id.to_string())
            .execute(&self.db)
            .await?;
        Ok(())
    }
}
