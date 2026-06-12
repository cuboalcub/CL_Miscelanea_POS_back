use axum::{
    extract::{State, Json as JsonExtractor},
    Json,
};
use chrono::Utc;

use crate::auth::Claims;
use crate::errors::AppError;
use crate::models::sync::{
    SyncBatchRequest, SyncBatchResponse, SyncErrorDetail, SyncOperationResult,
};
use crate::processors::sync_processor;
use crate::repositories::sync_repo;
use crate::state::AppState;

fn sugerencia_para(codigo: &str) -> Option<String> {
    match codigo {
        "CONFLICT" => Some(
            "Obtén los datos actualizados del servidor y aplica tus cambios \
             locales antes de reintentar."
                .to_string(),
        ),
        "VALIDACION" => {
            Some("Revisa los campos enviados. Corrige los datos localmente y reintenta.".to_string())
        }
        "ERROR_PROCESAMIENTO" | "ERROR_ACTUALIZACION" | "DB_ERROR" | "UNEXPECTED" => Some(
            "Reintenta con backoff exponencial: espera 1s, 2s, 4s, 8s... \
             (máximo 60s) entre cada intento."
                .to_string(),
        ),
        "OPERACION_NO_SOPORTADA" => {
            Some("Actualiza la aplicación a la versión más reciente.".to_string())
        }
        _ => None,
    }
}

pub async fn sync_batch(
    State(state): State<AppState>,
    claims: Claims,
    JsonExtractor(body): JsonExtractor<SyncBatchRequest>,
) -> Result<Json<SyncBatchResponse>, AppError> {
    if body.operaciones.is_empty() {
        return Err(AppError::BadRequest(
            "El batch debe contener al menos una operación".to_string(),
        ));
    }

    if body.operaciones.len() > 200 {
        return Err(AppError::BadRequest(
            "El batch no puede contener más de 200 operaciones por solicitud".to_string(),
        ));
    }

    for (i, op) in body.operaciones.iter().enumerate() {
        if op.cliente_id.is_empty() {
            return Err(AppError::BadRequest(format!(
                "La operación {} no tiene un cliente_id válido",
                i
            )));
        }
        if op.tipo.is_empty() {
            return Err(AppError::BadRequest(format!(
                "La operación {} no tiene un tipo válido",
                i
            )));
        }
        if op.entidad.is_empty() {
            return Err(AppError::BadRequest(format!(
                "La operación {} no tiene una entidad válida",
                i
            )));
        }
    }

    state.set_tenant_context(claims.empresa_id, claims.usuario_id).await.map_err(|e| AppError::DatabaseError(e))?;

    let resultados_db = sync_repo::insertar_batch(
        &state.db,
        body.dispositivo_id,
        body.sucursal_id,
        Some(claims.usuario_id),
        claims.empresa_id,
        &body.operaciones,
    )
    .await?;

    let operaciones_ids: Vec<_> = resultados_db
        .iter()
        .filter_map(|(_, res)| res.as_ref().ok().copied())
        .collect();

    let operaciones = sync_repo::obtener_por_ids(&state.db, &operaciones_ids).await?;

    let resultados_procesados = sync_processor::procesar_batch(&state.db, &operaciones, claims.empresa_id).await;

    use std::collections::HashMap;
    let mapa_procesados: HashMap<_, _> = resultados_procesados
        .into_iter()
        .map(|(id, estado, resultado)| (id, (estado, resultado)))
        .collect();

    let mut resultados = Vec::with_capacity(resultados_db.len());

    for (idx, res) in resultados_db {
        let cliente_id = body.operaciones[idx].cliente_id.clone();

        match res {
            Ok(sync_operation_id) => {
                let (estado_interno, resultado_valor) = mapa_procesados
                    .get(&sync_operation_id)
                    .cloned()
                    .unwrap_or_else(|| ("pendiente".to_string(), None));

                let estado_respuesta = match estado_interno.as_str() {
                    "procesado" => "ok",
                    "rechazado" => "error",
                    _ => "pendiente",
                };

                let error = resultado_valor.as_ref().and_then(|r| {
                    if estado_interno == "rechazado" {
                        let codigo = r
                            .get("codigo")
                            .and_then(|v| v.as_str())
                            .unwrap_or("UNKNOWN")
                            .to_string();
                        let mensaje = r
                            .get("mensaje")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Error desconocido")
                            .to_string();
                        let sugerencia = sugerencia_para(&codigo);

                        Some(SyncErrorDetail {
                            codigo,
                            mensaje,
                            sugerencia,
                        })
                    } else {
                        None
                    }
                });

                let data = match estado_interno.as_str() {
                    "procesado" => resultado_valor.clone(),
                    _ => None,
                };

                resultados.push(SyncOperationResult {
                    cliente_id,
                    estado: estado_respuesta.to_string(),
                    sync_operation_id: Some(sync_operation_id),
                    error,
                    data,
                });
            }
            Err(e) => {
                let (codigo, mensaje) = match &e {
                    AppError::DatabaseError(_) => ("DB_ERROR", "Error al guardar la operación"),
                    _ => ("UNEXPECTED", "Error inesperado"),
                };
                let sugerencia = sugerencia_para(codigo);
                resultados.push(SyncOperationResult {
                    cliente_id,
                    estado: "error".to_string(),
                    sync_operation_id: None,
                    error: Some(SyncErrorDetail {
                        codigo: codigo.to_string(),
                        mensaje: mensaje.to_string(),
                        sugerencia,
                    }),
                    data: None,
                });
            }
        }
    }

    let total = resultados.len();
    let ok_count = resultados.iter().filter(|r| r.estado == "ok").count();
    let error_count = resultados.iter().filter(|r| r.estado == "error").count();
    let primer_error = resultados
        .iter()
        .find(|r| r.error.is_some())
        .and_then(|r| {
            r.error.as_ref().map(|e| format!("{}: {}", e.codigo, e.mensaje))
        });

    let _ = sync_repo::insertar_audit_log(
        &state.db,
        body.dispositivo_id,
        body.sucursal_id,
        Some(claims.usuario_id),
        claims.empresa_id,
        total,
        ok_count,
        error_count,
        primer_error,
    )
    .await;

    Ok(Json(SyncBatchResponse {
        resultados,
        timestamp: Utc::now(),
    }))
}
