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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmpresaDto {
    pub nombre: Option<String>,
    pub rfc: Option<String>,
    pub regimen_fiscal: Option<String>,
    pub logo_url: Option<Option<String>>,
    pub subdominio: Option<String>,
}
