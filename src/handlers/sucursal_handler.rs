use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::NivelAcceso;
use crate::models::{CreateSucursalDto, Sucursal, UpdateSucursalDto};
use crate::repositories::sucursal_repo;
use crate::state::AppState;

pub async fn listar_sucursales(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Sucursal>>, AppError> {
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let sucursales = sucursal_repo::listar(&state.db, claims.empresa_id).await?;
    Ok(Json(sucursales))
}

pub async fn obtener_sucursal(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<Sucursal>, AppError> {
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let sucursal = sucursal_repo::obtener(&state.db, id, claims.empresa_id).await?;
    Ok(Json(sucursal))
}

pub async fn crear_sucursal(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateSucursalDto>,
) -> Result<(StatusCode, Json<Sucursal>), AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let sucursal = sucursal_repo::crear(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(sucursal)))
}

pub async fn actualizar_sucursal(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateSucursalDto>,
) -> Result<Json<Sucursal>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let sucursal = sucursal_repo::actualizar(&state.db, id, claims.empresa_id, payload).await?;
    Ok(Json(sucursal))
}

pub async fn eliminar_sucursal(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    sucursal_repo::eliminar(&state.db, id, claims.empresa_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
