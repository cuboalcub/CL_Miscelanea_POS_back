use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::{Compra, CompraResponse, CreateCompraRequest};
use crate::repositories::compra_repo;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct CompraQuery {
    pub sucursal_id: Uuid,
}

pub async fn crear_compra(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateCompraRequest>,
) -> Result<(StatusCode, Json<CompraResponse>), AppError> {
    let response =
        compra_repo::crear(&state.db, payload, claims.usuario_id, claims.empresa_id).await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn listar_compras(
    State(state): State<AppState>,
    claims: Claims,
    Query(query): Query<CompraQuery>,
) -> Result<Json<Vec<Compra>>, AppError> {
    let compras =
        compra_repo::listar_por_sucursal(&state.db, query.sucursal_id, claims.empresa_id).await?;
    Ok(Json(compras))
}

pub async fn obtener_compra(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<CompraResponse>, AppError> {
    let response = compra_repo::obtener(&state.db, id, claims.empresa_id).await?;
    Ok(Json(response))
}
