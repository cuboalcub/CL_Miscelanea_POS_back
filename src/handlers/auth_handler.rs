use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::auth;
use crate::errors::AppError;
use crate::models::NivelAcceso;
use crate::repositories::perfil_repo;
use crate::repositories::usuario_repo;
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

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UserInfo,
    pub perfil: PerfilInfo,
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
