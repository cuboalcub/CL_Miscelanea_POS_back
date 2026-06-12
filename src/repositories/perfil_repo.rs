use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreatePerfilDto, Perfil, UpdatePerfilDto};

pub async fn listar_por_usuario(pool: &PgPool, usuario_id: Uuid) -> Result<Vec<Perfil>, AppError> {
    let rows = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        FROM perfiles
        WHERE usuario_id = $1
        ORDER BY created_at ASC
        "#,
    )
    .bind(usuario_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn listar_por_empresa(pool: &PgPool, empresa_id: Uuid) -> Result<Vec<Perfil>, AppError> {
    let rows = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        FROM perfiles
        WHERE empresa_id = $1
        ORDER BY nivel ASC, created_at ASC
        "#,
    )
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<Perfil, AppError> {
    let row = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        FROM perfiles
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn crear(pool: &PgPool, dto: CreatePerfilDto) -> Result<Perfil, AppError> {
    let row = sqlx::query_as::<_, Perfil>(
        r#"
        INSERT INTO perfiles (usuario_id, empresa_id, sucursal_id, nivel)
        VALUES ($1, $2, $3, $4)
        RETURNING id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        "#,
    )
    .bind(dto.usuario_id)
    .bind(dto.empresa_id)
    .bind(dto.sucursal_id)
    .bind(dto.nivel)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(pool: &PgPool, id: Uuid, empresa_id: Uuid, dto: UpdatePerfilDto) -> Result<Perfil, AppError> {
    let row = sqlx::query_as::<_, Perfil>(
        r#"
        UPDATE perfiles SET
            nivel      = COALESCE($2, nivel),
            activo     = COALESCE($3, activo),
            updated_at = NOW()
        WHERE id = $1 AND empresa_id = $4
        RETURNING id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(dto.nivel)
    .bind(dto.activo)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn eliminar(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<(), AppError> {
    let resultado = sqlx::query("DELETE FROM perfiles WHERE id = $1 AND empresa_id = $2")
        .bind(id)
        .bind(empresa_id)
        .execute(pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Perfil con id {} no encontrado", id)));
    }

    Ok(())
}

pub async fn obtener_activo_por_usuario_empresa(
    pool: &PgPool,
    usuario_id: Uuid,
    empresa_id: Uuid,
) -> Result<Option<Perfil>, AppError> {
    let row = sqlx::query_as::<_, Perfil>(
        r#"
        SELECT id, usuario_id, empresa_id, sucursal_id, nivel, activo, created_at, updated_at
        FROM perfiles
        WHERE usuario_id = $1 AND empresa_id = $2 AND activo = true
        ORDER BY nivel ASC
        LIMIT 1
        "#,
    )
    .bind(usuario_id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}
