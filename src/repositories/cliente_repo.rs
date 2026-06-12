use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Cliente;

pub async fn insertar(pool: &PgPool, cliente: &Cliente) -> Result<Cliente, AppError> {
    let row = sqlx::query_as::<_, Cliente>(
        r#"
        INSERT INTO clientes (id, nombre, rfc, email, telefono, direccion, regimen_fiscal,
                              activo, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "#,
    )
    .bind(cliente.id)
    .bind(&cliente.nombre)
    .bind(&cliente.rfc)
    .bind(&cliente.email)
    .bind(&cliente.telefono)
    .bind(&cliente.direccion)
    .bind(&cliente.regimen_fiscal)
    .bind(cliente.activo)
    .bind(cliente.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(
    pool: &PgPool,
    id: Uuid,
    datos: &serde_json::Value,
) -> Result<Cliente, AppError> {
    let row = sqlx::query_as::<_, Cliente>(
        r#"
        UPDATE clientes SET
            nombre       = COALESCE(($2)::VARCHAR, nombre),
            rfc          = COALESCE(($3)::VARCHAR, rfc),
            email        = COALESCE(($4)::VARCHAR, email),
            telefono     = COALESCE(($5)::VARCHAR, telefono),
            direccion    = COALESCE(($6)::VARCHAR, direccion),
            regimen_fiscal = COALESCE(($7)::VARCHAR, regimen_fiscal),
            activo       = COALESCE(($8)::BOOLEAN, activo),
            updated_at   = NOW()
        WHERE id = $1 AND empresa_id = $9
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(datos.get("nombre").and_then(|v| v.as_str()))
    .bind(datos.get("rfc").and_then(|v| v.as_str()))
    .bind(datos.get("email").and_then(|v| v.as_str()))
    .bind(datos.get("telefono").and_then(|v| v.as_str()))
    .bind(datos.get("direccion").and_then(|v| v.as_str()))
    .bind(datos.get("regimen_fiscal").and_then(|v| v.as_str()))
    .bind(datos.get("activo").and_then(|v| v.as_bool()))
    .bind(
        datos
            .get("empresa_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()),
    )
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Cliente no encontrado".to_string()))
}

pub async fn eliminar(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
) -> Result<Cliente, AppError> {
    let row = sqlx::query_as::<_, Cliente>(
        r#"
        UPDATE clientes SET activo = FALSE, updated_at = NOW()
        WHERE id = $1 AND empresa_id = $2 AND activo = TRUE
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Cliente no encontrado".to_string()))
}
