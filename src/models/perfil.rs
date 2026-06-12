use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// Nivel de acceso que puede tener un usuario en un scope (empresa/sucursal).
/// El orden refleja privilegios de mayor a menor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "nivel_acceso", rename_all = "snake_case")]
pub enum NivelAcceso {
    SuperAdmin,
    Admin,
    Cajero,
    Consulta,
}

/// Perfil: registro pivot que vincula un usuario con una empresa
/// y opcionalmente con una sucursal específica.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Perfil {
    pub id: Uuid,
    pub usuario_id: Uuid,
    pub empresa_id: Uuid,
    /// None → acceso a toda la empresa; Some(id) → solo esa sucursal
    pub sucursal_id: Option<Uuid>,
    pub nivel: NivelAcceso,
    pub activo: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Para asignar (crear) un perfil a un usuario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePerfilDto {
    pub usuario_id: Uuid,
    pub empresa_id: Uuid,
    pub sucursal_id: Option<Uuid>,
    pub nivel: NivelAcceso,
}

/// Para modificar nivel o estado activo de un perfil existente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePerfilDto {
    pub nivel: Option<NivelAcceso>,
    pub activo: Option<bool>,
}
