use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateEmpresaDto, DuenoInfo, Empresa, UpdateEmpresaDto};

pub async fn listar(pool: &PgPool) -> Result<Vec<Empresa>, AppError> {
    let rows = sqlx::query_as::<_, Empresa>(
        r#"
        SELECT id, nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color, created_at, updated_at
        FROM empresas
        ORDER BY nombre ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid) -> Result<Empresa, AppError> {
    let row = sqlx::query_as::<_, Empresa>(
        r#"
        SELECT id, nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color, created_at, updated_at
        FROM empresas
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn crear(pool: &PgPool, dto: CreateEmpresaDto, owner_id: Option<Uuid>) -> Result<Empresa, AppError> {
    let row = sqlx::query_as::<_, Empresa>(
        r#"
        INSERT INTO empresas (nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color, created_at, updated_at
        "#,
    )
    .bind(dto.nombre)
    .bind(dto.rfc)
    .bind(dto.regimen_fiscal)
    .bind(dto.logo_url)
    .bind(dto.subdominio)
    .bind(owner_id)
    .bind(dto.color)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(pool: &PgPool, id: Uuid, dto: UpdateEmpresaDto) -> Result<Empresa, AppError> {
    let row = sqlx::query_as::<_, Empresa>(
        r#"
        UPDATE empresas SET
            nombre          = COALESCE($2, nombre),
            rfc             = COALESCE($3, rfc),
            regimen_fiscal  = COALESCE($4, regimen_fiscal),
            logo_url        = COALESCE($5, logo_url),
            subdominio      = COALESCE($6, subdominio),
            color           = COALESCE($7, color),
            updated_at      = NOW()
        WHERE id = $1
        RETURNING id, nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(dto.nombre)
    .bind(dto.rfc)
    .bind(dto.regimen_fiscal)
    .bind(dto.logo_url)
    .bind(dto.subdominio)
    .bind(dto.color)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn eliminar(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let resultado = sqlx::query("DELETE FROM empresas WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Empresa con id {} no encontrada", id)));
    }

    Ok(())
}

pub async fn listar_con_duenos(pool: &PgPool) -> Result<Vec<DuenoInfo>, AppError> {
    let rows = sqlx::query_as::<_, DuenoInfo>(
        r#"
        SELECT 
            e.id AS empresa_id,
            e.nombre AS empresa_nombre,
            e.subdominio AS empresa_subdominio,
            u.id AS dueno_id,
            u.nombre AS dueno_nombre,
            u.email AS dueno_email,
            COUNT(s.id) AS total_sucursales,
            e.created_at AS empresa_created_at
        FROM empresas e
        LEFT JOIN usuarios u ON u.id = e.owner_id
        LEFT JOIN sucursales s ON s.empresa_id = e.id
        GROUP BY e.id, e.nombre, e.subdominio, u.id, u.nombre, u.email, e.created_at
        ORDER BY e.nombre ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn buscar_por_subdominio(pool: &PgPool, subdominio: &str) -> Result<Empresa, AppError> {
    let row = sqlx::query_as::<_, Empresa>(
        r#"
        SELECT id, nombre, rfc, regimen_fiscal, logo_url, subdominio, owner_id, color, created_at, updated_at
        FROM empresas
        WHERE subdominio = $1
        "#,
    )
    .bind(subdominio)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
