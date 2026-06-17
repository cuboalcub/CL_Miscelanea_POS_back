use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Venta {
    pub id: Uuid,
    pub folio: String,
    pub cliente_id: Option<Uuid>,
    pub sucursal_id: Uuid,
    pub usuario_id: Uuid,
    pub subtotal: f64,
    pub iva: f64,
    pub total: f64,
    pub forma_pago: Option<String>,
    pub metodo_pago: Option<String>,
    pub uso_cfdi: Option<String>,
    pub estado: String,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VentaDetalle {
    pub id: Uuid,
    pub venta_id: Uuid,
    pub producto_id: Uuid,
    pub cantidad: f64,
    pub precio_unitario: f64,
    pub importe: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVentaItem {
    pub producto_id: Uuid,
    pub cantidad: f64,
    pub precio_unitario: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVentaRequest {
    pub cliente_id: Option<Uuid>,
    pub sucursal_id: Uuid,
    pub forma_pago: Option<String>,
    pub metodo_pago: Option<String>,
    pub uso_cfdi: Option<String>,
    pub items: Vec<CreateVentaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VentaResponse {
    pub venta: Venta,
    pub detalle: Vec<VentaDetalle>,
}
