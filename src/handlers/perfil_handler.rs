use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::NivelAcceso;
use crate::models::{CreatePerfilDto, Perfil, UpdatePerfilDto};
use crate::repositories::perfil_repo;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct PerfilQuery {
    pub usuario_id: Option<Uuid>,
    pub empresa_id: Option<Uuid>,
}

pub async fn listar_perfiles(
    State(state): State<AppState>,
    claims: Claims,
    Query(params): Query<PerfilQuery>,
) -> Result<Json<Vec<Perfil>>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let perfiles = match (params.usuario_id, params.empresa_id) {
        (Some(uid), _) => perfil_repo::listar_por_usuario(&state.db, uid).await?,
        (_, Some(eid)) => {
            if eid != claims.empresa_id {
                return Err(AppError::Forbidden("No tienes acceso a esta empresa".to_string()));
            }
            perfil_repo::listar_por_empresa(&state.db, eid).await?
        }
        _ => {
            return Err(AppError::BadRequest(
                "Se requiere usuario_id o empresa_id como query param".to_string(),
            ))
        }
    };
    Ok(Json(perfiles))
}

pub async fn obtener_perfil(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<Perfil>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let perfil = perfil_repo::obtener(&state.db, id, claims.empresa_id).await?;
    Ok(Json(perfil))
}

pub async fn crear_perfil(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreatePerfilDto>,
) -> Result<(StatusCode, Json<Perfil>), AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin]).await?;
    if payload.empresa_id != claims.empresa_id {
        return Err(AppError::Forbidden("No puedes asignar perfiles a otra empresa".to_string()));
    }
    let perfil = perfil_repo::crear(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(perfil)))
}

pub async fn actualizar_perfil(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePerfilDto>,
) -> Result<Json<Perfil>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin]).await?;
    let perfil = perfil_repo::actualizar(&state.db, id, claims.empresa_id, payload).await?;
    Ok(Json(perfil))
}

pub async fn eliminar_perfil(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin]).await?;
    perfil_repo::eliminar(&state.db, id, claims.empresa_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
