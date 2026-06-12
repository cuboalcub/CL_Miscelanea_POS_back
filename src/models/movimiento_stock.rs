use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TipoMovimiento {
    Entrada,
    Salida,
    Ajuste,
    Venta,
    Traspaso,
}

impl TipoMovimiento {
    pub fn as_str(&self) -> &'static str {
        match self {
            TipoMovimiento::Entrada => "ENTRADA",
            TipoMovimiento::Salida => "SALIDA",
            TipoMovimiento::Ajuste => "AJUSTE",
            TipoMovimiento::Venta => "VENTA",
            TipoMovimiento::Traspaso => "TRASPASO",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MovimientoStock {
    pub id: Uuid,
    pub inventario_id: Uuid,
    pub tipo_movimiento: String,
    pub cantidad: f64,
    pub stock_antes: f64,
    pub stock_despues: f64,
    pub referencia_tipo: Option<String>,
    pub referencia_id: Option<Uuid>,
    pub motivo: Option<String>,
    pub usuario_id: Option<Uuid>,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMovimientoStockDto {
    pub inventario_id: Uuid,
    pub tipo: TipoMovimiento,
    pub cantidad: f64,
    #[serde(default)]
    pub stock_antes: Option<f64>,
    #[serde(default)]
    pub stock_despues: Option<f64>,
    pub referencia_tipo: Option<String>,
    pub referencia_id: Option<Uuid>,
    pub motivo: Option<String>,
    pub usuario_id: Option<Uuid>,
    pub empresa_id: Uuid,
}
