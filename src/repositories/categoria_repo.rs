use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::Categoria;

pub async fn listar_por_empresa(pool: &PgPool, empresa_id: Uuid) -> Result<Vec<Categoria>, AppError> {
    let rows = sqlx::query_as::<_, Categoria>(
        r#"
        SELECT id, nombre, empresa_id, created_at, updated_at
        FROM categorias
        WHERE empresa_id = $1
        ORDER BY nombre
        "#,
    )
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener_por_id(pool: &PgPool, id: Uuid) -> Result<Categoria, AppError> {
    let row = sqlx::query_as::<_, Categoria>(
        r#"
        SELECT id, nombre, empresa_id, created_at, updated_at
        FROM categorias
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Categoría no encontrada".to_string()))
}

pub async fn insertar(
    pool: &PgPool,
    nombre: &str,
    empresa_id: Uuid,
) -> Result<Categoria, AppError> {
    let row = sqlx::query_as::<_, Categoria>(
        r#"
        INSERT INTO categorias (nombre, empresa_id)
        VALUES ($1, $2)
        RETURNING id, nombre, empresa_id, created_at, updated_at
        "#,
    )
    .bind(nombre)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(
    pool: &PgPool,
    id: Uuid,
    nombre: &str,
    empresa_id: Uuid,
) -> Result<Categoria, AppError> {
    let row = sqlx::query_as::<_, Categoria>(
        r#"
        UPDATE categorias SET nombre = $1, updated_at = NOW()
        WHERE id = $2 AND empresa_id = $3
        RETURNING id, nombre, empresa_id, created_at, updated_at
        "#,
    )
    .bind(nombre)
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Categoría no encontrada".to_string()))
}

pub async fn eliminar(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query(
        r#"
        DELETE FROM categorias
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Categoría no encontrada".to_string()));
    }

    Ok(())
}
