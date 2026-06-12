use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateInventarioDto, Inventario, UpdateInventarioDto};

pub async fn listar_por_sucursal(
    pool: &PgPool,
    sucursal_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<Inventario>, AppError> {
    let rows = sqlx::query_as::<_, Inventario>(
        r#"
        SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
               stock_maximo, ubicacion, empresa_id, created_at, updated_at
        FROM inventario
        WHERE sucursal_id = $1 AND empresa_id = $2
        ORDER BY producto_id
        "#,
    )
    .bind(sucursal_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn listar_por_producto(
    pool: &PgPool,
    producto_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<Inventario>, AppError> {
    let rows = sqlx::query_as::<_, Inventario>(
        r#"
        SELECT i.id, i.producto_id, i.sucursal_id, i.stock_actual, i.stock_minimo,
               i.stock_maximo, i.ubicacion, i.empresa_id, i.created_at, i.updated_at
        FROM inventario i
        INNER JOIN productos p ON p.id = i.producto_id
        WHERE i.producto_id = $1 AND p.empresa_id = $2
        ORDER BY i.sucursal_id
        "#,
    )
    .bind(producto_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<Inventario, AppError> {
    let row = sqlx::query_as::<_, Inventario>(
        r#"
        SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
               stock_maximo, ubicacion, empresa_id, created_at, updated_at
        FROM inventario
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn obtener_por_producto_sucursal(
    pool: &PgPool,
    producto_id: Uuid,
    sucursal_id: Uuid,
    empresa_id: Uuid,
) -> Result<Option<Inventario>, AppError> {
    let row = sqlx::query_as::<_, Inventario>(
        r#"
        SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
               stock_maximo, ubicacion, empresa_id, created_at, updated_at
        FROM inventario
        WHERE producto_id = $1 AND sucursal_id = $2 AND empresa_id = $3
        "#,
    )
    .bind(producto_id)
    .bind(sucursal_id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn crear(
    pool: &PgPool,
    dto: CreateInventarioDto,
) -> Result<Inventario, AppError> {
    let row = sqlx::query_as::<_, Inventario>(
        r#"
        INSERT INTO inventario (producto_id, sucursal_id, stock_actual,
                                stock_minimo, stock_maximo, ubicacion, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, producto_id, sucursal_id, stock_actual, stock_minimo,
                  stock_maximo, ubicacion, empresa_id, created_at, updated_at
        "#,
    )
    .bind(dto.producto_id)
    .bind(dto.sucursal_id)
    .bind(dto.stock_actual.unwrap_or(0.0))
    .bind(dto.stock_minimo.unwrap_or(0.0))
    .bind(dto.stock_maximo.unwrap_or(0.0))
    .bind(dto.ubicacion)
    .bind(dto.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
    dto: UpdateInventarioDto,
) -> Result<Inventario, AppError> {
    let row = sqlx::query_as::<_, Inventario>(
        r#"
        UPDATE inventario SET
            stock_actual = COALESCE($2, stock_actual),
            stock_minimo = COALESCE($3, stock_minimo),
            stock_maximo = COALESCE($4, stock_maximo),
            ubicacion    = COALESCE($5, ubicacion),
            updated_at   = NOW()
        WHERE id = $1 AND empresa_id = $6
        RETURNING id, producto_id, sucursal_id, stock_actual, stock_minimo,
                  stock_maximo, ubicacion, empresa_id, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(dto.stock_actual)
    .bind(dto.stock_minimo)
    .bind(dto.stock_maximo)
    .bind(dto.ubicacion)
    .bind(empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn descontar_stock(
    pool: &PgPool,
    id: Uuid,
    cantidad: f64,
    empresa_id: Uuid,
) -> Result<Inventario, AppError> {
    let row = sqlx::query_as::<_, Inventario>(
        r#"
        UPDATE inventario
        SET stock_actual = stock_actual - $1,
            updated_at   = NOW()
        WHERE id = $2 AND stock_actual >= $1 AND empresa_id = $3
        RETURNING id, producto_id, sucursal_id, stock_actual, stock_minimo,
                  stock_maximo, ubicacion, empresa_id, created_at, updated_at
        "#,
    )
    .bind(cantidad)
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    match row {
        Some(inv) => Ok(inv),
        None => {
            let existente = sqlx::query_as::<_, Inventario>(
                r#"
                SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
                       stock_maximo, ubicacion, empresa_id, created_at, updated_at
                FROM inventario
                WHERE id = $1
                "#,
            )
            .bind(id)
            .fetch_optional(pool)
            .await?;

            match existente {
                Some(inv) => Err(AppError::InsufficientStock(format!(
                    "Stock insuficiente para el producto {}. Disponible: {}, solicitado: {}",
                    inv.producto_id, inv.stock_actual, cantidad
                ))),
                None => Err(AppError::NotFound(format!(
                    "Registro de inventario {} no encontrado",
                    id
                ))),
            }
        }
    }
}

pub async fn validar_stock_para_venta(
    pool: &PgPool,
    sucursal_id: Uuid,
    empresa_id: Uuid,
    items: &[(Uuid, f64)],
) -> Result<(), AppError> {
    for (producto_id, cantidad) in items {
        let inventario = sqlx::query_as::<_, Inventario>(
            r#"
            SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
                   stock_maximo, ubicacion, empresa_id, created_at, updated_at
            FROM inventario
            WHERE producto_id = $1 AND sucursal_id = $2 AND empresa_id = $3
            "#,
        )
        .bind(producto_id)
        .bind(sucursal_id)
        .bind(empresa_id)
        .fetch_optional(pool)
        .await?;

        match inventario {
            Some(inv) if inv.stock_actual >= *cantidad => continue,
            Some(inv) => {
                return Err(AppError::InsufficientStock(format!(
                    "Stock insuficiente para el producto {}. \
                     Disponible: {}, requerido: {}",
                    producto_id, inv.stock_actual, cantidad
                )));
            }
            None => {
                return Err(AppError::InsufficientStock(format!(
                    "El producto {} no tiene registro de inventario en la sucursal {}",
                    producto_id, sucursal_id
                )));
            }
        }
    }

    Ok(())
}

pub async fn eliminar(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<(), AppError> {
    let resultado = sqlx::query("DELETE FROM inventario WHERE id = $1 AND empresa_id = $2")
        .bind(id)
        .bind(empresa_id)
        .execute(pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Inventario con id {} no encontrado",
            id
        )));
    }

    Ok(())
}
