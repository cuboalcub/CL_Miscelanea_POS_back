use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Compra {
    pub id: Uuid,
    pub folio: String,
    pub proveedor: Option<String>,
    pub sucursal_id: Uuid,
    pub usuario_id: Uuid,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CompraDetalle {
    pub id: Uuid,
    pub compra_id: Uuid,
    pub producto_id: Uuid,
    pub cantidad: f64,
    pub precio_unitario: f64,
    pub importe: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompraItem {
    pub producto_id: Uuid,
    pub cantidad: f64,
    pub precio_unitario: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompraRequest {
    pub proveedor: Option<String>,
    pub sucursal_id: Uuid,
    pub items: Vec<CreateCompraItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompraResponse {
    pub compra: Compra,
    pub detalle: Vec<CompraDetalle>,
}
