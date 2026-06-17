use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::{CreateVentaRequest, Venta, VentaResponse};
use crate::repositories::venta_repo;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct VentaQuery {
    pub sucursal_id: Uuid,
}

pub async fn crear_venta(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateVentaRequest>,
) -> Result<(StatusCode, Json<VentaResponse>), AppError> {
    let response = venta_repo::crear(&state.db, payload, claims.usuario_id, claims.empresa_id)
        .await?;
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn listar_ventas(
    State(state): State<AppState>,
    claims: Claims,
    Query(query): Query<VentaQuery>,
) -> Result<Json<Vec<Venta>>, AppError> {
    let ventas =
        venta_repo::listar_por_sucursal(&state.db, query.sucursal_id, claims.empresa_id).await?;
    Ok(Json(ventas))
}

pub async fn obtener_venta(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<VentaResponse>, AppError> {
    let response = venta_repo::obtener(&state.db, id, claims.empresa_id).await?;
    Ok(Json(response))
}

pub async fn cancelar_venta(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<VentaResponse>, AppError> {
    let response = venta_repo::cancelar(&state.db, id, claims.empresa_id).await?;
    Ok(Json(response))
}
