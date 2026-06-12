use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgPool;
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{Inventario, MovimientoStock};

// ─── Tipos de alerta ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize)]
pub enum StockAlertType {
    #[serde(rename = "stock_bajo")]
    StockBajo,
    #[serde(rename = "stock_agotado")]
    StockAgotado,
    #[serde(rename = "stock_restaurado")]
    StockRestaurado,
}

#[derive(Debug, Clone, Serialize)]
pub struct StockAlertEvent {
    pub alert_type: StockAlertType,
    pub inventario_id: Uuid,
    pub producto_id: Uuid,
    pub sucursal_id: Uuid,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub timestamp: DateTime<Utc>,
}

// ─── Evaluación de alertas ──────────────────────────────────────────────────

/// Evalúa si el movimiento dispara una alerta de stock y la envía al canal.
/// Es seguro llamarlo fire-and-forget: los errores se loguean y se ignoran.
pub async fn notify_after_movement(
    pool: &PgPool,
    tx: &broadcast::Sender<StockAlertEvent>,
    movimiento: &MovimientoStock,
) {
    let alert = match check_stock_level(pool, movimiento).await {
        Ok(Some(a)) => a,
        Ok(None) => return,
        Err(e) => {
            tracing::error!("Error evaluando alerta de stock: {:?}", e);
            return;
        }
    };

    let _ = tx.send(alert);
}

/// Lee el inventario y decide si se debe disparar una alerta.
async fn check_stock_level(
    pool: &PgPool,
    movimiento: &MovimientoStock,
) -> Result<Option<StockAlertEvent>, AppError> {
    let inv = sqlx::query_as::<_, Inventario>(
        r#"
        SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
               stock_maximo, ubicacion, empresa_id, created_at, updated_at
        FROM inventario
        WHERE id = $1
        "#,
    )
    .bind(movimiento.inventario_id)
    .fetch_optional(pool)
    .await?;

    let inv = match inv {
        Some(i) => i,
        None => return Ok(None),
    };

    let alert_type = if movimiento.stock_despues <= 0.0 {
        StockAlertType::StockAgotado
    } else if movimiento.stock_despues <= inv.stock_minimo {
        StockAlertType::StockBajo
    } else {
        return Ok(None);
    };

    Ok(Some(StockAlertEvent {
        alert_type,
        inventario_id: movimiento.inventario_id,
        producto_id: inv.producto_id,
        sucursal_id: inv.sucursal_id,
        stock_actual: movimiento.stock_despues,
        stock_minimo: inv.stock_minimo,
        timestamp: Utc::now(),
    }))
}
