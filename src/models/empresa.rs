use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Empresa {
    pub id: Uuid,
    pub nombre: String,
    pub rfc: String,
    pub regimen_fiscal: String,
    pub logo_url: Option<String>,
    pub subdominio: String,
    pub owner_id: Option<Uuid>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmpresaDto {
    pub nombre: String,
    pub rfc: String,
    pub regimen_fiscal: String,
    pub logo_url: Option<String>,
    pub subdominio: String,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdateEmpresaDto {
    pub nombre: Option<String>,
    pub rfc: Option<String>,
    pub regimen_fiscal: Option<String>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub logo_url: Option<Option<String>>,
    pub subdominio: Option<String>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub color: Option<Option<String>>,
}

/// Información de un dueño de empresa para el panel de admin de plataforma
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DuenoInfo {
    pub empresa_id: Uuid,
    pub empresa_nombre: String,
    pub empresa_subdominio: String,
    pub dueno_id: Option<Uuid>,
    pub dueno_nombre: Option<String>,
    pub dueno_email: Option<String>,
    pub total_sucursales: i64,
    pub empresa_created_at: DateTime<Utc>,
}
