use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Sucursal {
    pub id: Uuid,
    pub nombre: String,
    pub direccion_completa: String,
    pub telefono: Option<String>,
    pub encargado: Option<String>,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSucursalDto {
    pub nombre: String,
    pub direccion_completa: String,
    pub telefono: Option<String>,
    pub encargado: Option<String>,
    pub empresa_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSucursalDto {
    pub nombre: Option<String>,
    pub direccion_completa: Option<String>,
    pub telefono: Option<Option<String>>,
    pub encargado: Option<Option<String>>,
    pub empresa_id: Option<Uuid>,
}
