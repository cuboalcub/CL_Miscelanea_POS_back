use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inventario {
    pub id: Uuid,
    pub producto_id: Uuid,
    pub sucursal_id: Uuid,
    pub stock_actual: f64,
    pub stock_minimo: f64,
    pub stock_maximo: f64,
    pub ubicacion: Option<String>,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInventarioDto {
    pub producto_id: Uuid,
    pub sucursal_id: Uuid,
    pub stock_actual: Option<f64>,
    pub stock_minimo: Option<f64>,
    pub stock_maximo: Option<f64>,
    pub ubicacion: Option<String>,
    pub empresa_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInventarioDto {
    pub stock_actual: Option<f64>,
    pub stock_minimo: Option<f64>,
    pub stock_maximo: Option<f64>,
    pub ubicacion: Option<Option<String>>,
}
