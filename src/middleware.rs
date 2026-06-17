use axum::{
    body::Body,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use uuid::Uuid;

use crate::errors::AppError;
use crate::state::AppState;

pub async fn log_request_response(request: Request<Body>, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let start = Instant::now();

    let response = next.run(request).await;

    let duration = start.elapsed();
    let status = response.status();

    if status.is_server_error() {
        tracing::error!(
            method = %method,
            path = %uri,
            status = %status,
            duration_ms = duration.as_millis(),
            "Error en la respuesta"
        );
    } else if status.is_client_error() {
        tracing::warn!(
            method = %method,
            path = %uri,
            status = %status,
            duration_ms = duration.as_millis(),
            "Solicitud inválida"
        );
    } else {
        tracing::info!(
            method = %method,
            path = %uri,
            status = %status,
            duration_ms = duration.as_millis(),
            "Respuesta exitosa"
        );
    }

    response
}

#[derive(Debug, Clone)]
pub struct TenantContext {
    pub empresa_id: Uuid,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for TenantContext
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // 1. Intentar obtener empresa_id del header X-Empresa-ID (dev/testing)
        if let Some(empresa_id_str) = parts
            .headers
            .get("X-Empresa-ID")
            .and_then(|v| v.to_str().ok())
            .filter(|s| !s.is_empty())
        {
            let empresa_id = Uuid::parse_str(empresa_id_str)
                .map_err(|_| {
                    tracing::error!("X-Empresa-ID inválido: {}", empresa_id_str);
                    AppError::BadRequest("X-Empresa-ID inválido".to_string())
                })?;
            return Ok(TenantContext { empresa_id });
        }

        // 2. Resolver por subdominio desde el Host header
        let host = parts
            .headers
            .get("Host")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                tracing::error!("Cabecera Host faltante para resolución de tenant");
                AppError::BadRequest("Cabecera Host requerida".to_string())
            })?;

        let subdominio = host
            .split('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        if subdominio.is_empty() || subdominio == "localhost" || subdominio.contains("127.0.0.1") {
            tracing::warn!("No se pudo resolver tenant desde subdominio, host: {}", host);
            return Err(AppError::BadRequest(
                "No se pudo resolver el tenant desde el subdominio. Usa X-Empresa-ID para desarrollo.".to_string(),
            ));
        }

        let empresa = sqlx::query_scalar::<_, Uuid>(
            r#"SELECT id FROM empresas WHERE subdominio = $1"#,
        )
        .bind(&subdominio)
        .fetch_optional(&app_state.db)
        .await
        .map_err(|e| {
            tracing::error!("Error en BD resolviendo tenant por subdominio '{}': {:?}", subdominio, e);
            AppError::InternalError("Error al resolver tenant".to_string())
        })?
        .ok_or_else(|| {
            tracing::warn!("Empresa con subdominio '{}' no encontrada", subdominio);
            AppError::NotFound(format!("Empresa con subdominio '{}' no encontrada", subdominio))
        })?;

        Ok(TenantContext { empresa_id: empresa })
    }
}
