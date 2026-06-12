use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::NivelAcceso;
use crate::models::{CreateUsuarioDto, UpdateUsuarioDto, Usuario};
use crate::repositories::usuario_repo;
use crate::state::AppState;

pub async fn listar_usuarios(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Usuario>>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let usuarios = usuario_repo::listar(&state.db, claims.empresa_id).await?;
    Ok(Json(usuarios))
}

pub async fn obtener_usuario(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<Usuario>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let usuario = usuario_repo::obtener(&state.db, id).await?;
    Ok(Json(usuario))
}

pub async fn crear_usuario(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateUsuarioDto>,
) -> Result<(StatusCode, Json<Usuario>), AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let usuario = usuario_repo::crear(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(usuario)))
}

pub async fn actualizar_usuario(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUsuarioDto>,
) -> Result<Json<Usuario>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    let usuario = usuario_repo::actualizar(&state.db, id, payload).await?;
    Ok(Json(usuario))
}

pub async fn eliminar_usuario(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    usuario_repo::eliminar(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn obtener_mi_perfil(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<crate::models::ProfileResponse>, AppError> {
    let perfil_completo = usuario_repo::obtener_perfil_completo(&state.db, claims.usuario_id).await?;
    Ok(Json(perfil_completo))
}
