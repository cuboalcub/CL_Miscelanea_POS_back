use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Modelo de Clave de Producto o Servicio oficial del SAT (c_ClaveProdServ).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatClaveProdServ {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Unidad de Medida oficial del SAT (c_ClaveUnidad).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatUnidadMedida {
    pub clave: String,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Forma de Pago oficial del SAT (c_FormaPago).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatFormaPago {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Método de Pago oficial del SAT (c_MetodoPago).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatMetodoPago {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Uso del CFDI oficial del SAT (c_UsoCFDI).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatUsoCfdi {
    pub clave: String,
    pub descripcion: String,
    pub regimen_fiscal_aplica: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Régimen Fiscal oficial del SAT (c_RegimenFiscal).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatRegimenFiscal {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Tipo de Comprobante oficial del SAT (c_TipoComprobante).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatTipoComprobante {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

/// Modelo de Clave de Exportación oficial del SAT (c_Exportacion).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SatExportacion {
    pub clave: String,
    pub descripcion: String,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
}

