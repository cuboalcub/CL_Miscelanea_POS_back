use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Usuario {
    pub id: Uuid,
    pub nombre: String,
    pub email: String,
    /// Nunca se serializa en respuestas HTTP
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub activo: bool,
    /// Indica si es administrador de la plataforma SaaS (ve todas las empresas)
    #[serde(skip_serializing)]
    pub es_admin_plataforma: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUsuarioDto {
    pub nombre: String,
    pub email: String,
    /// Contraseña en texto plano; el repositorio aplica el hash antes de guardar
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUsuarioDto {
    pub nombre: Option<String>,
    pub email: Option<String>,
    pub activo: Option<bool>,
}
