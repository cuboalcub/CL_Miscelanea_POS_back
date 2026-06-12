use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateSucursalDto, Sucursal, UpdateSucursalDto};

pub async fn listar(pool: &PgPool, empresa_id: Uuid) -> Result<Vec<Sucursal>, AppError> {
    let rows = sqlx::query_as::<_, Sucursal>(
        r#"
        SELECT id, nombre, direccion_completa, telefono, encargado, empresa_id, created_at, updated_at
        FROM sucursales
        WHERE empresa_id = $1
        ORDER BY nombre ASC
        "#,
    )
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<Sucursal, AppError> {
    let row = sqlx::query_as::<_, Sucursal>(
        r#"
        SELECT id, nombre, direccion_completa, telefono, encargado, empresa_id, created_at, updated_at
        FROM sucursales
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn crear(pool: &PgPool, dto: CreateSucursalDto) -> Result<Sucursal, AppError> {
    let row = sqlx::query_as::<_, Sucursal>(
        r#"
        INSERT INTO sucursales (nombre, direccion_completa, telefono, encargado, empresa_id)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, nombre, direccion_completa, telefono, encargado, empresa_id, created_at, updated_at
        "#,
    )
    .bind(dto.nombre)
    .bind(dto.direccion_completa)
    .bind(dto.telefono)
    .bind(dto.encargado)
    .bind(dto.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(pool: &PgPool, id: Uuid, empresa_id: Uuid, dto: UpdateSucursalDto) -> Result<Sucursal, AppError> {
    let row = sqlx::query_as::<_, Sucursal>(
        r#"
        UPDATE sucursales SET
            nombre              = COALESCE($2, nombre),
            direccion_completa  = COALESCE($3, direccion_completa),
            telefono            = COALESCE($4, telefono),
            encargado           = COALESCE($5, encargado),
            empresa_id          = COALESCE($6, empresa_id),
            updated_at          = NOW()
        WHERE id = $1 AND empresa_id = $7
        RETURNING id, nombre, direccion_completa, telefono, encargado, empresa_id, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(dto.nombre)
    .bind(dto.direccion_completa)
    .bind(dto.telefono)
    .bind(dto.encargado)
    .bind(dto.empresa_id)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn eliminar(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<(), AppError> {
    let resultado = sqlx::query("DELETE FROM sucursales WHERE id = $1 AND empresa_id = $2")
        .bind(id)
        .bind(empresa_id)
        .execute(pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Sucursal con id {} no encontrada", id)));
    }

    Ok(())
}
