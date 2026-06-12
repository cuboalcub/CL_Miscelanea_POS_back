use axum::{extract::State, Json};

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::DuenoInfo;
use crate::repositories::empresa_repo;
use crate::state::AppState;

/// GET /admin/duenos — Lista todas las empresas con información de sus dueños.
/// Solo accesible para administradores de plataforma.
pub async fn listar_duenos(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<DuenoInfo>>, AppError> {
    claims.require_admin_plataforma(&state).await?;
    let duenos = empresa_repo::listar_con_duenos(&state.db).await?;
    Ok(Json(duenos))
}
