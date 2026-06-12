use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::sync::{SyncOperation, SyncOperationRequest};

pub async fn insertar_audit_log(
    pool: &PgPool,
    dispositivo_id: Uuid,
    sucursal_id: Uuid,
    usuario_id: Option<Uuid>,
    empresa_id: Uuid,
    total: usize,
    ok: usize,
    error: usize,
    primer_error: Option<String>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO sync_audit_log
            (dispositivo_id, sucursal_id, usuario_id, empresa_id, total_operaciones,
             operaciones_ok, operaciones_error, primer_error)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
    )
    .bind(dispositivo_id)
    .bind(sucursal_id)
    .bind(usuario_id)
    .bind(empresa_id)
    .bind(total as i32)
    .bind(ok as i32)
    .bind(error as i32)
    .bind(primer_error)
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Error insertando sync_audit_log: {:?}", e);
        AppError::DatabaseError(e)
    })?;

    Ok(())
}

pub async fn insertar_batch(
    pool: &PgPool,
    dispositivo_id: Uuid,
    sucursal_id: Uuid,
    usuario_id: Option<Uuid>,
    empresa_id: Uuid,
    operaciones: &[SyncOperationRequest],
) -> Result<Vec<(usize, Result<Uuid, AppError>)>, AppError> {
    if operaciones.is_empty() {
        return Ok(Vec::new());
    }

    let mut qb = sqlx::QueryBuilder::new(
        "INSERT INTO sync_operations \
         (dispositivo_id, sucursal_id, usuario_id, empresa_id, cliente_id, \
          tipo_operacion, entidad, datos, ocurrido_en) ",
    );

    qb.push_values(operaciones.iter(), |mut b, op| {
        b.push_bind(dispositivo_id)
            .push_bind(sucursal_id)
            .push_bind(usuario_id)
            .push_bind(empresa_id)
            .push_bind(&op.cliente_id)
            .push_bind(&op.tipo)
            .push_bind(&op.entidad)
            .push_bind(&op.datos)
            .push_bind(op.timestamp);
    });

    qb.push(" ON CONFLICT (dispositivo_id, cliente_id) DO NOTHING RETURNING id, cliente_id");

    let rows = qb
        .build_query_as::<(Uuid, String)>()
        .fetch_all(pool)
        .await
        .map_err(|e| {
            tracing::error!("Error en inserción masiva sync_operations: {:?}", e);
            AppError::DatabaseError(e)
        })?;

    let insertados: std::collections::HashMap<&str, Uuid> = rows
        .iter()
        .map(|(id, cid)| (cid.as_str(), *id))
        .collect();

    let mut resultados = Vec::with_capacity(operaciones.len());

    for (i, op) in operaciones.iter().enumerate() {
        match insertados.get(op.cliente_id.as_str()) {
            Some(id) => resultados.push((i, Ok(*id))),
            None => {
                let existente = sqlx::query_scalar::<_, Uuid>(
                    r#"
                    SELECT id FROM sync_operations
                    WHERE dispositivo_id = $1 AND cliente_id = $2
                    "#,
                )
                .bind(dispositivo_id)
                .bind(&op.cliente_id)
                .fetch_optional(pool)
                .await
                .map_err(AppError::DatabaseError)?;

                match existente {
                    Some(id) => resultados.push((i, Ok(id))),
                    None => resultados.push((
                        i,
                        Err(AppError::InternalError(
                            "Conflicto de idempotencia sin registro previo".to_string(),
                        )),
                    )),
                }
            }
        }
    }

    Ok(resultados)
}

pub async fn obtener_por_ids(
    pool: &PgPool,
    ids: &[Uuid],
) -> Result<Vec<SyncOperation>, AppError> {
    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let rows = sqlx::query_as::<_, SyncOperation>(
        r#"
        SELECT id, dispositivo_id, sucursal_id, usuario_id, empresa_id, cliente_id,
               tipo_operacion, entidad, datos, estado, resultado,
               ocurrido_en, created_at, procesado_en
        FROM sync_operations
        WHERE id = ANY($1)
        ORDER BY ocurrido_en ASC
        "#,
    )
    .bind(ids)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
