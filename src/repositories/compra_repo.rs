use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{Compra, CompraDetalle, CompraResponse, CreateCompraRequest, Inventario};

pub async fn crear(
    pool: &PgPool,
    dto: CreateCompraRequest,
    usuario_id: Uuid,
    empresa_id: Uuid,
) -> Result<CompraResponse, AppError> {
    let subtotal: f64 = dto
        .items
        .iter()
        .map(|i| i.cantidad * i.precio_unitario)
        .sum();
    let iva = subtotal * 0.16;
    let total = subtotal + iva;
    let folio = format!("C-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let compra_id = Uuid::new_v4();

    let mut tx = pool.begin().await?;

    let compra = sqlx::query_as::<_, Compra>(
        r#"
        INSERT INTO compras (id, folio, proveedor, sucursal_id, usuario_id,
                             subtotal, iva, total, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, folio, proveedor, sucursal_id, usuario_id,
                  subtotal, iva, total, empresa_id, created_at, updated_at
        "#,
    )
    .bind(compra_id)
    .bind(&folio)
    .bind(&dto.proveedor)
    .bind(dto.sucursal_id)
    .bind(usuario_id)
    .bind(subtotal)
    .bind(iva)
    .bind(total)
    .bind(empresa_id)
    .fetch_one(&mut *tx)
    .await?;

    let mut detalle = Vec::with_capacity(dto.items.len());

    for item in &dto.items {
        let importe = item.cantidad * item.precio_unitario;
        let detalle_id = Uuid::new_v4();

        let detalle_item = sqlx::query_as::<_, CompraDetalle>(
            r#"
            INSERT INTO compras_detalle (id, compra_id, producto_id, cantidad,
                                         precio_unitario, importe)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, compra_id, producto_id, cantidad,
                      precio_unitario, importe, created_at
            "#,
        )
        .bind(detalle_id)
        .bind(compra_id)
        .bind(item.producto_id)
        .bind(item.cantidad)
        .bind(item.precio_unitario)
        .bind(importe)
        .fetch_one(&mut *tx)
        .await?;

        let inv = sqlx::query_as::<_, Inventario>(
            r#"
            SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
                   stock_maximo, ubicacion, empresa_id, created_at, updated_at
            FROM inventario
            WHERE producto_id = $1 AND sucursal_id = $2 AND empresa_id = $3
            "#,
        )
        .bind(item.producto_id)
        .bind(dto.sucursal_id)
        .bind(empresa_id)
        .fetch_optional(&mut *tx)
        .await?;

        let inventario_id = match inv {
            Some(i) => i.id,
            None => {
                let nuevo_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO inventario (id, producto_id, sucursal_id, stock_actual,
                                            stock_minimo, stock_maximo, empresa_id)
                    VALUES ($1, $2, $3, 0, 0, 0, $4)
                    "#,
                )
                .bind(nuevo_id)
                .bind(item.producto_id)
                .bind(dto.sucursal_id)
                .bind(empresa_id)
                .execute(&mut *tx)
                .await?;
                nuevo_id
            }
        };

        sqlx::query(
            r#"
            INSERT INTO movimientos_stock (inventario_id, tipo_movimiento, cantidad,
                                           stock_antes, stock_despues, referencia_tipo,
                                           referencia_id, motivo, usuario_id, empresa_id)
            VALUES ($1, 'ENTRADA', $2, 0, 0, 'COMPRA', $3, $4, $5, $6)
            "#,
        )
        .bind(inventario_id)
        .bind(item.cantidad)
        .bind(compra_id)
        .bind(format!("Compra {}", folio))
        .bind(usuario_id)
        .bind(empresa_id)
        .execute(&mut *tx)
        .await?;

        detalle.push(detalle_item);
    }

    tx.commit().await?;

    Ok(CompraResponse { compra, detalle })
}

pub async fn listar_por_sucursal(
    pool: &PgPool,
    sucursal_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<Compra>, AppError> {
    let rows = sqlx::query_as::<_, Compra>(
        r#"
        SELECT id, folio, proveedor, sucursal_id, usuario_id,
               subtotal, iva, total, empresa_id, created_at, updated_at
        FROM compras
        WHERE sucursal_id = $1 AND empresa_id = $2
        ORDER BY created_at DESC
        "#,
    )
    .bind(sucursal_id)
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
) -> Result<CompraResponse, AppError> {
    let compra = sqlx::query_as::<_, Compra>(
        r#"
        SELECT id, folio, proveedor, sucursal_id, usuario_id,
               subtotal, iva, total, empresa_id, created_at, updated_at
        FROM compras
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Compra no encontrada".to_string()))?;

    let detalle = sqlx::query_as::<_, CompraDetalle>(
        r#"
        SELECT id, compra_id, producto_id, cantidad,
               precio_unitario, importe, created_at
        FROM compras_detalle
        WHERE compra_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(CompraResponse { compra, detalle })
}
