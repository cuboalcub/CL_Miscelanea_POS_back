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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProductoDto {
    pub sku: Option<String>,
    pub codigo_barras: Option<String>,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub precio_venta: f64,
    pub precio_compra: f64,
    pub sat_clave: Option<String>,
    pub sat_unidad: Option<String>,
    pub categoria_id: Option<Uuid>,
    pub iva_incluido: Option<bool>,
    pub empresa_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProductoDto {
    pub sku: Option<Option<String>>,
    pub codigo_barras: Option<Option<String>>,
    pub nombre: Option<String>,
    pub descripcion: Option<Option<String>>,
    pub precio_venta: Option<f64>,
    pub precio_compra: Option<f64>,
    pub sat_clave: Option<Option<String>>,
    pub sat_unidad: Option<Option<String>>,
    pub categoria_id: Option<Option<Uuid>>,
    pub iva_incluido: Option<bool>,
    pub activo: Option<bool>,
}
