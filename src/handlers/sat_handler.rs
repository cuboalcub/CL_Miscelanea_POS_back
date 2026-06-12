use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::errors::AppError;
use crate::models::{
    SatClaveProdServ, SatExportacion, SatFormaPago, SatMetodoPago, SatRegimenFiscal,
    SatTipoComprobante, SatUsoCfdi,
};
use crate::repositories::sat_repo;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct BuscarQuery {
    pub q: Option<String>,
}

/// Endpoint: GET /sat/claves?q=...
/// Busca en el catálogo oficial de claves del SAT por descripción o por clave numérica.
pub async fn buscar_claves(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatClaveProdServ>>, AppError> {
    let q = query.q.unwrap_or_default();
    let claves = sat_repo::buscar(&state.db, &q).await?;
    Ok(Json(claves))
}

/// Endpoint: GET /sat/unidades?q=...
/// Busca en el catálogo oficial de unidades de medida del SAT por nombre o por clave alfanumérica.
pub async fn buscar_unidades(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<crate::models::SatUnidadMedida>>, AppError> {
    let q = query.q.unwrap_or_default();
    let unidades = sat_repo::buscar_unidades(&state.db, &q).await?;
    Ok(Json(unidades))
}

/// Endpoint: GET /sat/formas-pago?q=...
/// Busca en el catálogo oficial de formas de pago del SAT por descripción o por clave.
pub async fn buscar_formas_pago(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatFormaPago>>, AppError> {
    let q = query.q.unwrap_or_default();
    let formas_pago = sat_repo::buscar_formas_pago(&state.db, &q).await?;
    Ok(Json(formas_pago))
}

/// Endpoint: GET /sat/metodos-pago?q=...
pub async fn buscar_metodos_pago(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatMetodoPago>>, AppError> {
    let q = query.q.unwrap_or_default();
    let metodos = sat_repo::buscar_metodos_pago(&state.db, &q).await?;
    Ok(Json(metodos))
}

/// Endpoint: GET /sat/usos-cfdi?q=...
pub async fn buscar_usos_cfdi(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatUsoCfdi>>, AppError> {
    let q = query.q.unwrap_or_default();
    let usos = sat_repo::buscar_usos_cfdi(&state.db, &q).await?;
    Ok(Json(usos))
}

/// Endpoint: GET /sat/regimenes-fiscales?q=...
pub async fn buscar_regimenes_fiscales(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatRegimenFiscal>>, AppError> {
    let q = query.q.unwrap_or_default();
    let regimenes = sat_repo::buscar_regimenes_fiscales(&state.db, &q).await?;
    Ok(Json(regimenes))
}

/// Endpoint: GET /sat/tipos-comprobante?q=...
pub async fn buscar_tipos_comprobante(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatTipoComprobante>>, AppError> {
    let q = query.q.unwrap_or_default();
    let tipos = sat_repo::buscar_tipos_comprobante(&state.db, &q).await?;
    Ok(Json(tipos))
}

/// Endpoint: GET /sat/exportacion?q=...
pub async fn buscar_exportacion(
    State(state): State<AppState>,
    Query(query): Query<BuscarQuery>,
) -> Result<Json<Vec<SatExportacion>>, AppError> {
    let q = query.q.unwrap_or_default();
    let exportacion = sat_repo::buscar_exportacion(&state.db, &q).await?;
    Ok(Json(exportacion))
}

