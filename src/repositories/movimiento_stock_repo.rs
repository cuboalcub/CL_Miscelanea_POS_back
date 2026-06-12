use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateMovimientoStockDto, MovimientoStock};

pub async fn registrar(
    pool: &PgPool,
    dto: CreateMovimientoStockDto,
) -> Result<MovimientoStock, AppError> {
    let row = sqlx::query_as::<_, MovimientoStock>(
        r#"
        INSERT INTO movimientos_stock (inventario_id, tipo_movimiento, cantidad,
                                       stock_antes, stock_despues, referencia_tipo,
                                       referencia_id, motivo, usuario_id, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, inventario_id, tipo_movimiento, cantidad, stock_antes,
                  stock_despues, referencia_tipo, referencia_id, motivo,
                  usuario_id, empresa_id, created_at
        "#,
    )
    .bind(dto.inventario_id)
    .bind(dto.tipo.as_str())
    .bind(dto.cantidad)
    .bind(dto.stock_antes.unwrap_or(0.0))
    .bind(dto.stock_despues.unwrap_or(0.0))
    .bind(&dto.referencia_tipo)
    .bind(dto.referencia_id)
    .bind(&dto.motivo)
    .bind(dto.usuario_id)
    .bind(dto.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn listar_por_inventario(
    pool: &PgPool,
    inventario_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<MovimientoStock>, AppError> {
    let rows = sqlx::query_as::<_, MovimientoStock>(
        r#"
        SELECT id, inventario_id, tipo_movimiento, cantidad, stock_antes,
               stock_despues, referencia_tipo, referencia_id, motivo,
               usuario_id, empresa_id, created_at
        FROM movimientos_stock
        WHERE inventario_id = $1 AND empresa_id = $2
        ORDER BY created_at DESC
        "#,
    )
    .bind(inventario_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn listar_por_producto_sucursal(
    pool: &PgPool,
    producto_id: Uuid,
    sucursal_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<MovimientoStock>, AppError> {
    let rows = sqlx::query_as::<_, MovimientoStock>(
        r#"
        SELECT m.id, m.inventario_id, m.tipo_movimiento, m.cantidad, m.stock_antes,
               m.stock_despues, m.referencia_tipo, m.referencia_id, m.motivo,
               m.usuario_id, m.empresa_id, m.created_at
        FROM movimientos_stock m
        JOIN inventario i ON i.id = m.inventario_id
        WHERE i.producto_id = $1 AND i.sucursal_id = $2 AND m.empresa_id = $3
        ORDER BY m.created_at DESC
        "#,
    )
    .bind(producto_id)
    .bind(sucursal_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn listar_por_referencia(
    pool: &PgPool,
    referencia_tipo: &str,
    referencia_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<MovimientoStock>, AppError> {
    let rows = sqlx::query_as::<_, MovimientoStock>(
        r#"
        SELECT id, inventario_id, tipo_movimiento, cantidad, stock_antes,
               stock_despues, referencia_tipo, referencia_id, motivo,
               usuario_id, empresa_id, created_at
        FROM movimientos_stock
        WHERE referencia_tipo = $1 AND referencia_id = $2 AND empresa_id = $3
        ORDER BY created_at DESC
        "#,
    )
    .bind(referencia_tipo)
    .bind(referencia_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
