use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Modelo de Cliente sincronizable desde dispositivo móvil.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cliente {
    pub id: Uuid,
    pub nombre: String,
    pub rfc: Option<String>,
    pub email: Option<String>,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub regimen_fiscal: Option<String>,
    pub activo: bool,
    pub empresa_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
