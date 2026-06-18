use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth;
use crate::errors::AppError;
use crate::models::{CreateEmpresaDto, CreateUsuarioDto, NivelAcceso};
use crate::repositories::{empresa_repo, perfil_repo, usuario_repo};
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
    pub empresa_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub nombre: String,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct PerfilInfo {
    pub empresa_id: Uuid,
    pub nivel: NivelAcceso,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDto {
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub empresa_nombre: String,
    pub empresa_rfc: String,
    pub empresa_regimen_fiscal: String,
    pub empresa_subdominio: String,
    pub empresa_logo_url: Option<String>,
    pub empresa_color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmpresaInfo {
    pub id: Uuid,
    pub nombre: String,
    pub rfc: String,
    pub regimen_fiscal: String,
    pub subdominio: String,
    pub logo_url: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub token: String,
    pub usuario: UserInfo,
    pub empresa: EmpresaInfo,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UserInfo,
    pub perfil: PerfilInfo,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<(StatusCode, Json<RegisterResponse>), AppError> {
    if usuario_repo::buscar_por_email(&state.db, &payload.email).await.is_ok() {
        return Err(AppError::BadRequest("El email ya está registrado".to_string()));
    }

    if empresa_repo::buscar_por_subdominio(&state.db, &payload.empresa_subdominio).await.is_ok() {
        return Err(AppError::BadRequest("El subdominio ya está en uso".to_string()));
    }

    auth::validar_contrasena_fuerte(&payload.password)?;

    let usuario = usuario_repo::crear(
        &state.db,
        CreateUsuarioDto {
            nombre: payload.nombre,
            email: payload.email,
            password: payload.password,
        },
    )
    .await?;

    let empresa = empresa_repo::crear(
        &state.db,
        CreateEmpresaDto {
            nombre: payload.empresa_nombre,
            rfc: payload.empresa_rfc,
            regimen_fiscal: payload.empresa_regimen_fiscal,
            logo_url: payload.empresa_logo_url,
            subdominio: payload.empresa_subdominio,
            color: payload.empresa_color,
        },
        Some(usuario.id),
    )
    .await?;

    let perfil = perfil_repo::crear(
        &state.db,
        crate::models::CreatePerfilDto {
            usuario_id: usuario.id,
            empresa_id: empresa.id,
            sucursal_id: None,
            nivel: NivelAcceso::Admin,
        },
    )
    .await?;

    let token = auth::generar_jwt(
        &state,
        usuario.id,
        &usuario.email,
        empresa.id,
        perfil.nivel,
    )?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterResponse {
            token,
            usuario: UserInfo {
                id: usuario.id,
                nombre: usuario.nombre,
                email: usuario.email,
            },
            empresa: EmpresaInfo {
                id: empresa.id,
                nombre: empresa.nombre,
                rfc: empresa.rfc,
                regimen_fiscal: empresa.regimen_fiscal,
                subdominio: empresa.subdominio,
                logo_url: empresa.logo_url,
                color: empresa.color,
            },
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<LoginResponse>, AppError> {
    let usuario = usuario_repo::buscar_por_email(&state.db, &payload.email).await?;

    if !usuario.activo {
        return Err(AppError::Unauthorized("Usuario inactivo".to_string()));
    }

    let password_valida = if usuario.password_hash.starts_with("HASH_PLACEHOLDER:") {
        let stored_plain = usuario.password_hash.trim_start_matches("HASH_PLACEHOLDER:");
        if stored_plain != payload.password {
            return Err(AppError::Unauthorized("Credenciales inválidas".to_string()));
        }
        let nuevo_hash = auth::hashear_password(&payload.password)?;
        usuario_repo::actualizar_password_hash(&state.db, usuario.id, &nuevo_hash).await?;
        true
    } else {
        auth::verificar_password(&payload.password, &usuario.password_hash)?
    };

    if !password_valida {
        return Err(AppError::Unauthorized("Credenciales inválidas".to_string()));
    }

    let perfiles = perfil_repo::listar_por_usuario(&state.db, usuario.id).await?;

    let perfil_activo = if let Some(empresa_id) = payload.empresa_id {
        perfiles.iter().find(|p| p.empresa_id == empresa_id && p.activo)
    } else {
        perfiles.iter().find(|p| p.activo)
    };

    let perfil = perfil_activo.ok_or_else(|| {
        AppError::Forbidden("No tienes un perfil activo en ninguna empresa".to_string())
    })?;

    let token = auth::generar_jwt(
        &state,
        usuario.id,
        &usuario.email,
        perfil.empresa_id,
        perfil.nivel.clone(),
    )?;

    Ok(Json(LoginResponse {
        token,
        usuario: UserInfo {
            id: usuario.id,
            nombre: usuario.nombre,
            email: usuario.email,
        },
        perfil: PerfilInfo {
            empresa_id: perfil.empresa_id,
            nivel: perfil.nivel.clone(),
        },
    }))
}
