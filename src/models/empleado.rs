use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Perfil de empleado con datos adicionales, vinculado 1:1 con el auth nativo (Supabase).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PerfilEmpleado {
    pub id: Uuid,
    pub nombre_completo: String,
    pub telefono: Option<String>,
    pub puesto: Option<String>,
    pub curp: Option<String>,
    pub nss: Option<String>,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub fecha_ingreso: NaiveDate,
    pub direccion: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePerfilEmpleadoDto {
    pub id: Uuid,
    pub nombre_completo: String,
    pub telefono: Option<String>,
    pub puesto: Option<String>,
    pub curp: Option<String>,
    pub nss: Option<String>,
    pub fecha_nacimiento: Option<NaiveDate>,
    pub fecha_ingreso: Option<NaiveDate>,
    pub direccion: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct UpdatePerfilEmpleadoDto {
    pub nombre_completo: Option<String>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub telefono: Option<Option<String>>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub puesto: Option<Option<String>>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub curp: Option<Option<String>>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub nss: Option<Option<String>>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub fecha_nacimiento: Option<Option<NaiveDate>>,
    pub fecha_ingreso: Option<NaiveDate>,
    #[serde(deserialize_with = "super::deserialize_present_option")]
    pub direccion: Option<Option<String>>,
}
