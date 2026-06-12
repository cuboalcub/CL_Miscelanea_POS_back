use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::{CreateEmpresaDto, Empresa, UpdateEmpresaDto};
use crate::repositories::empresa_repo;
use crate::state::AppState;

pub async fn listar_empresas(
    State(state): State<AppState>,
    _claims: Claims,
) -> Result<Json<Vec<Empresa>>, AppError> {
    let empresas = empresa_repo::listar(&state.db).await?;
    Ok(Json(empresas))
}

pub async fn obtener_empresa(
    State(state): State<AppState>,
    _claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<Json<Empresa>, AppError> {
    let empresa = empresa_repo::obtener(&state.db, id).await?;
    Ok(Json(empresa))
}

pub async fn crear_empresa(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateEmpresaDto>,
) -> Result<(StatusCode, Json<Empresa>), AppError> {
    claims.require_super_admin(&state).await?;
    let empresa = empresa_repo::crear(&state.db, payload, Some(claims.usuario_id)).await?;

    let perfil_dto = crate::models::CreatePerfilDto {
        usuario_id: claims.usuario_id,
        empresa_id: empresa.id,
        sucursal_id: None,
        nivel: crate::models::NivelAcceso::SuperAdmin,
    };
    crate::repositories::perfil_repo::crear(&state.db, perfil_dto).await?;

    Ok((StatusCode::CREATED, Json(empresa)))
}

pub async fn actualizar_empresa(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateEmpresaDto>,
) -> Result<Json<Empresa>, AppError> {
    claims.require_super_admin(&state).await?;
    let empresa = empresa_repo::actualizar(&state.db, id, payload).await?;
    Ok(Json(empresa))
}

pub async fn eliminar_empresa(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    claims.require_super_admin(&state).await?;
    empresa_repo::eliminar(&state.db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
