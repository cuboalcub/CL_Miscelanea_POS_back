use serde::Serialize;
use uuid::Uuid;

use crate::models::{NivelAcceso, PerfilEmpleado, Usuario};

/// Representa el detalle completo de un rol asignado, incluyendo información del tenant.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct RolAsignadoDetalle {
    pub perfil_id: Uuid,
    pub empresa_id: Uuid,
    pub empresa_nombre: String,
    pub sucursal_id: Option<Uuid>,
    pub sucursal_nombre: Option<String>,
    pub nivel: NivelAcceso,
}

/// Respuesta completa del endpoint GET /profile
#[derive(Debug, Clone, Serialize)]
pub struct ProfileResponse {
    pub usuario: Usuario,
    pub datos_empleado: Option<PerfilEmpleado>,
    pub roles_asignados: Vec<RolAsignadoDetalle>,
}
