use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::{CreateProductoDto, NivelAcceso, Producto, UpdateProductoDto};
use crate::repositories::producto_repo;
use crate::state::AppState;

pub async fn listar_productos(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Producto>>, AppError> {
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let productos = producto_repo::listar(&state.db, claims.empresa_id).await?;
    Ok(Json(productos))
}

pub async fn obtener_producto(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<Producto>, AppError> {
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let producto = producto_repo::obtener(&state.db, id, claims.empresa_id).await?;
    Ok(Json(producto))
}

pub async fn crear_producto(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateProductoDto>,
) -> Result<(StatusCode, Json<Producto>), AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let producto = producto_repo::crear(&state.db, payload).await?;
    Ok((StatusCode::CREATED, Json(producto)))
}

pub async fn actualizar_producto(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProductoDto>,
) -> Result<Json<Producto>, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    let producto = producto_repo::actualizar_con_dto(&state.db, id, claims.empresa_id, payload).await?;
    Ok(Json(producto))
}

pub async fn eliminar_producto(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    claims.require_role(&state, claims.empresa_id, &[NivelAcceso::SuperAdmin, NivelAcceso::Admin]).await?;
    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;
    producto_repo::eliminar(&state.db, id, claims.empresa_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
