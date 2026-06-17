use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::NivelAcceso;
use crate::repositories::perfil_repo;
use crate::state::AppState;
use tracing::error;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub email: String,
    pub empresa_id: Uuid,
    pub nivel: NivelAcceso,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct Claims {
    pub usuario_id: Uuid,
    pub empresa_id: Uuid,
    pub nivel_acceso: NivelAcceso,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or_else(|| {
                error!("Solicitud sin cabecera Authorization");
                AppError::Unauthorized("Falta cabecera de autenticación Authorization".to_string())
            })?
            .to_str()
            .map_err(|_| {
                error!("Cabecera Authorization con formato inválido");
                AppError::BadRequest("Formato de cabecera inválido".to_string())
            })?;

        if !auth_header.starts_with("Bearer ") {
            error!("Cabecera Authorization no es de tipo Bearer");
            return Err(AppError::Unauthorized("El token de autorización debe ser de tipo Bearer".to_string()));
        }

        let token = &auth_header[7..];

        let decoding_key = DecodingKey::from_secret(app_state.jwt_secret.as_bytes());
        match decode::<JwtClaims>(token, &decoding_key, &Validation::default()) {
            Ok(token_data) => {
                Ok(Claims {
                    usuario_id: token_data.claims.sub,
                    empresa_id: token_data.claims.empresa_id,
                    nivel_acceso: token_data.claims.nivel,
                })
            }
            Err(e) => {
                error!("Token JWT inválido o expirado: {:?}", e);
                Err(AppError::Unauthorized(
                    "Token JWT inválido o expirado".to_string(),
                ))
            }
        }
    }
}

impl Claims {
    pub async fn require_role(
        &self,
        _state: &AppState,
        empresa_id: Uuid,
        roles_permitidos: &[NivelAcceso],
    ) -> Result<(), AppError> {
        if self.empresa_id != empresa_id {
            return Err(AppError::Forbidden(
                "No tienes acceso a esta empresa".to_string(),
            ));
        }
        if !roles_permitidos.contains(&self.nivel_acceso) {
            return Err(AppError::Forbidden(
                "No cuentas con el nivel de acceso requerido para esta empresa".to_string(),
            ));
        }
        Ok(())
    }

    pub async fn require_super_admin(&self, state: &AppState) -> Result<(), AppError> {
        let perfiles = perfil_repo::listar_por_usuario(&state.db, self.usuario_id).await?;

        let es_super_admin = perfiles.iter().any(|p| {
            p.activo && p.nivel == NivelAcceso::SuperAdmin
        });

        if !es_super_admin {
            return Err(AppError::Forbidden(
                "Esta operación requiere privilegios de nivel Super Administrador".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn require_admin_plataforma(&self, state: &AppState) -> Result<(), AppError> {
        let usuario = crate::repositories::usuario_repo::obtener(&state.db, self.usuario_id).await?;

        if !usuario.es_admin_plataforma {
            return Err(AppError::Forbidden(
                "Esta operación requiere privilegios de Administrador de Plataforma".to_string(),
            ));
        }

        Ok(())
    }
}

pub fn generar_jwt(
    state: &AppState,
    usuario_id: Uuid,
    email: &str,
    empresa_id: Uuid,
    nivel: NivelAcceso,
) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expiration_hours = state.jwt_expiration_hours;
    let exp = (now + chrono::Duration::hours(expiration_hours)).timestamp() as usize;

    let claims = JwtClaims {
        sub: usuario_id,
        email: email.to_string(),
        empresa_id,
        nivel,
        iat: now.timestamp() as usize,
        exp,
    };

    let encoding_key = EncodingKey::from_secret(state.jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| AppError::InternalError(format!("Error generando JWT: {}", e)))
}

pub fn hashear_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("Error hasheando contraseña: {}", e)))?;
    Ok(hash.to_string())
}

pub fn verificar_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| AppError::InternalError(format!("Error parseando hash: {}", e)))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

pub fn validar_contrasena_fuerte(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::BadRequest(
            "La contraseña debe tener al menos 8 caracteres de longitud".to_string(),
        ));
    }

    let tiene_mayuscula = password.chars().any(|c| c.is_uppercase());
    let tiene_minuscula = password.chars().any(|c| c.is_lowercase());
    let tiene_numero = password.chars().any(|c| c.is_numeric());
    let tiene_especial = password.chars().any(|c| !c.is_alphanumeric());

    if !tiene_mayuscula {
        return Err(AppError::BadRequest(
            "La contraseña debe incluir al menos una letra mayúscula".to_string(),
        ));
    }
    if !tiene_minuscula {
        return Err(AppError::BadRequest(
            "La contraseña debe incluir al menos una letra minúscula".to_string(),
        ));
    }
    if !tiene_numero {
        return Err(AppError::BadRequest(
            "La contraseña debe incluir al menos un dígito numérico".to_string(),
        ));
    }
    if !tiene_especial {
        return Err(AppError::BadRequest(
            "La contraseña debe incluir al menos un carácter especial (ej. @, $, !, %, #, *, ?, &)".to_string(),
        ));
    }

    Ok(())
}
