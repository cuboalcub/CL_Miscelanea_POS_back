use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Modelo de Producto sincronizable desde dispositivo móvil.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Producto {
    pub id: Uuid,
    pub sku: Option<String>,
    pub codigo_barras: Option<String>,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta: f64,
    pub precio_compra: f64,
    pub sat_clave: Option<String>,
    pub sat_unidad: Option<String>,
    pub categoria_id: Option<Uuid>,
    pub iva_incluido: bool,
    pub activo: bool,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
