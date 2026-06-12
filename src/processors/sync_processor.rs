use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::sync::SyncOperation;
use crate::repositories::{cliente_repo, producto_repo};

#[derive(Debug)]
pub enum SyncProcessingError {
    Validacion(String),
    Referencia(String),
    Conflicto(String),
    RepoError(AppError),
    OperacionNoSoportada(String, String),
}

impl SyncProcessingError {
    fn codigo(&self) -> &str {
        match self {
            SyncProcessingError::Validacion(_) => "VALIDACION",
            SyncProcessingError::Referencia(_) => "REFERENCIA_INVALIDA",
            SyncProcessingError::Conflicto(_) => "CONFLICT",
            SyncProcessingError::RepoError(_) => "ERROR_PROCESAMIENTO",
            SyncProcessingError::OperacionNoSoportada(_, _) => "OPERACION_NO_SOPORTADA",
        }
    }

    fn mensaje(&self) -> String {
        match self {
            SyncProcessingError::Validacion(m) => m.clone(),
            SyncProcessingError::Referencia(m) => m.clone(),
            SyncProcessingError::Conflicto(m) => m.clone(),
            SyncProcessingError::RepoError(e) => format!("{:?}", e),
            SyncProcessingError::OperacionNoSoportada(e, t) => {
                format!("Operación '{}' no soportada para entidad '{}'", t, e)
            }
        }
    }
}

impl From<AppError> for SyncProcessingError {
    fn from(e: AppError) -> Self {
        SyncProcessingError::RepoError(e)
    }
}

pub async fn procesar(
    pool: &PgPool,
    op: &SyncOperation,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let entidad = op.entidad.as_str();
    let tipo = op.tipo_operacion.as_str();

    if entidad == "clientes" {
        if tipo == "insertar" {
            return procesar_insert_cliente(pool, &op.datos, empresa_id).await;
        }
        if tipo == "actualizar" {
            return procesar_update_cliente(pool, op.ocurrido_en, &op.datos, empresa_id).await;
        }
        if tipo == "eliminar" {
            return procesar_delete_cliente(pool, &op.datos, empresa_id).await;
        }
    }

    if entidad == "productos" {
        if tipo == "insertar" {
            return procesar_insert_producto(pool, &op.datos, empresa_id).await;
        }
        if tipo == "actualizar" {
            return procesar_update_producto(pool, op.ocurrido_en, &op.datos, empresa_id).await;
        }
        if tipo == "eliminar" {
            return procesar_delete_producto(pool, &op.datos, empresa_id).await;
        }
    }

    Err(SyncProcessingError::OperacionNoSoportada(
        entidad.to_string(),
        tipo.to_string(),
    ))
}

pub async fn procesar_batch(
    pool: &PgPool,
    operaciones: &[SyncOperation],
    empresa_id: Uuid,
) -> Vec<(Uuid, String, Option<serde_json::Value>)> {
    let mut resultados = Vec::with_capacity(operaciones.len());

    for op in operaciones {
        let (estado, resultado) = match procesar(pool, op, empresa_id).await {
            Ok(valor) => ("procesado".to_string(), Some(valor)),
            Err(e) => (
                "rechazado".to_string(),
                Some(serde_json::json!({
                    "codigo": e.codigo(),
                    "mensaje": e.mensaje()
                })),
            ),
        };

        if let Err(e) = sqlx::query(
            r#"
            UPDATE sync_operations
            SET estado = $1, resultado = $2, procesado_en = NOW()
            WHERE id = $3
            "#,
        )
        .bind(&estado)
        .bind(&resultado)
        .bind(op.id)
        .execute(pool)
        .await
        {
            tracing::error!(
                "Error crítico actualizando sync_operation {} ({}): {:?}",
                op.id, op.cliente_id, e
            );

            resultados.push((
                op.id,
                "rechazado".to_string(),
                Some(serde_json::json!({
                    "codigo": "ERROR_ACTUALIZACION",
                    "mensaje": "No se pudo registrar el resultado de la operación en el servidor"
                })),
            ));
            continue;
        }

        resultados.push((op.id, estado, resultado));
    }

    resultados
}

fn campo_requerido(datos: &serde_json::Value, campo: &str) -> Result<String, SyncProcessingError> {
    datos
        .get(campo)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            SyncProcessingError::Validacion(format!("Campo requerido '{}' no encontrado", campo))
        })
}

fn campo_requerido_uuid(
    datos: &serde_json::Value,
    campo: &str,
) -> Result<Uuid, SyncProcessingError> {
    datos
        .get(campo)
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| {
            SyncProcessingError::Validacion(format!(
                "Campo requerido '{}' no encontrado o UUID inválido",
                campo
            ))
        })
}

async fn procesar_insert_cliente(
    pool: &PgPool,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;
    let nombre = campo_requerido(datos, "nombre")?;

    let cliente = crate::models::Cliente {
        id,
        nombre,
        rfc: Some(campo_requerido(datos, "rfc")?),
        email: datos
            .get("email")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        telefono: datos
            .get("telefono")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        direccion: datos
            .get("direccion")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        regimen_fiscal: datos
            .get("regimen_fiscal")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        activo: true,
        empresa_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let creado = cliente_repo::insertar(pool, &cliente).await?;

    Ok(serde_json::json!({
        "id": creado.id,
        "nombre": creado.nombre,
        "server_created_at": creado.created_at,
        "server_updated_at": creado.updated_at,
    }))
}

async fn procesar_update_cliente(
    pool: &PgPool,
    client_ts: DateTime<Utc>,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;

    let existente = sqlx::query_as::<_, crate::models::Cliente>(
        "SELECT * FROM clientes WHERE id = $1 AND empresa_id = $2",
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| SyncProcessingError::RepoError(AppError::DatabaseError(e)))?
    .ok_or_else(|| {
        SyncProcessingError::Validacion("Cliente no encontrado".to_string())
    })?;

    if client_ts <= existente.updated_at {
        return Err(SyncProcessingError::Conflicto(
            "El servidor tiene una versión más reciente del cliente".to_string(),
        ));
    }

    let actualizado = cliente_repo::actualizar(pool, id, datos).await?;

    Ok(serde_json::json!({
        "id": actualizado.id,
        "nombre": actualizado.nombre,
        "server_updated_at": actualizado.updated_at,
    }))
}

async fn procesar_delete_cliente(
    pool: &PgPool,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;

    let eliminado = cliente_repo::eliminar(pool, id, empresa_id).await?;

    Ok(serde_json::json!({
        "id": eliminado.id,
        "eliminado": true,
        "server_updated_at": eliminado.updated_at,
    }))
}

async fn procesar_insert_producto(
    pool: &PgPool,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;
    let nombre = campo_requerido(datos, "nombre")?;

    let producto = crate::models::Producto {
        id,
        sku: datos
            .get("sku")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        codigo_barras: Some(campo_requerido(datos, "codigo_barras")?),
        nombre,
        descripcion: datos
            .get("descripcion")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
        precio_venta: datos
            .get("precio_venta")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        precio_compra: datos
            .get("precio_compra")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0),
        sat_clave: Some(campo_requerido(datos, "sat_clave")?),
        sat_unidad: Some(campo_requerido(datos, "sat_unidad")?),
        categoria_id: datos
            .get("categoria_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()),
        iva_incluido: datos
            .get("iva_incluido")
            .and_then(|v| v.as_bool())
            .unwrap_or(true),
        activo: true,
        empresa_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let creado = producto_repo::insertar(pool, &producto).await?;

    Ok(serde_json::json!({
        "id": creado.id,
        "nombre": creado.nombre,
        "server_created_at": creado.created_at,
        "server_updated_at": creado.updated_at,
    }))
}

async fn procesar_update_producto(
    pool: &PgPool,
    client_ts: DateTime<Utc>,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;

    let existente = sqlx::query_as::<_, crate::models::Producto>(
        "SELECT * FROM productos WHERE id = $1 AND empresa_id = $2",
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| SyncProcessingError::RepoError(AppError::DatabaseError(e)))?
    .ok_or_else(|| {
        SyncProcessingError::Validacion("Producto no encontrado".to_string())
    })?;

    if client_ts <= existente.updated_at {
        return Err(SyncProcessingError::Conflicto(
            "El servidor tiene una versión más reciente del producto".to_string(),
        ));
    }

    let actualizado = producto_repo::actualizar(pool, id, datos).await?;

    Ok(serde_json::json!({
        "id": actualizado.id,
        "nombre": actualizado.nombre,
        "server_updated_at": actualizado.updated_at,
    }))
}

async fn procesar_delete_producto(
    pool: &PgPool,
    datos: &serde_json::Value,
    empresa_id: Uuid,
) -> Result<serde_json::Value, SyncProcessingError> {
    let id = campo_requerido_uuid(datos, "id")?;

    let eliminado = producto_repo::eliminar(pool, id, empresa_id).await?;

    Ok(serde_json::json!({
        "id": eliminado.id,
        "eliminado": true,
        "server_updated_at": eliminado.updated_at,
    }))
}
