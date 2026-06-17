use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{
    CreateVentaRequest, Inventario, Venta, VentaDetalle, VentaResponse,
};
use crate::repositories::inventario_repo;

pub async fn crear(
    pool: &PgPool,
    dto: CreateVentaRequest,
    usuario_id: Uuid,
    empresa_id: Uuid,
) -> Result<VentaResponse, AppError> {
    let stock_items: Vec<(Uuid, f64)> =
        dto.items.iter().map(|i| (i.producto_id, i.cantidad)).collect();
    inventario_repo::validar_stock_para_venta(pool, dto.sucursal_id, empresa_id, &stock_items)
        .await?;

    let subtotal: f64 = dto
        .items
        .iter()
        .map(|i| i.cantidad * i.precio_unitario)
        .sum();
    let iva = subtotal * 0.16;
    let total = subtotal + iva;
    let folio = format!("V-{}", Utc::now().format("%Y%m%d%H%M%S"));
    let venta_id = Uuid::new_v4();

    let mut tx = pool.begin().await?;

    let venta = sqlx::query_as::<_, Venta>(
        r#"
        INSERT INTO ventas (id, folio, cliente_id, sucursal_id, usuario_id,
                            subtotal, iva, total, forma_pago, metodo_pago,
                            uso_cfdi, estado, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, 'completada', $12)
        RETURNING id, folio, cliente_id, sucursal_id, usuario_id,
                  subtotal, iva, total, forma_pago, metodo_pago,
                  uso_cfdi, estado, empresa_id, created_at, updated_at
        "#,
    )
    .bind(venta_id)
    .bind(&folio)
    .bind(dto.cliente_id)
    .bind(dto.sucursal_id)
    .bind(usuario_id)
    .bind(subtotal)
    .bind(iva)
    .bind(total)
    .bind(&dto.forma_pago)
    .bind(&dto.metodo_pago)
    .bind(&dto.uso_cfdi)
    .bind(empresa_id)
    .fetch_one(&mut *tx)
    .await?;

    let mut detalle = Vec::with_capacity(dto.items.len());

    for item in &dto.items {
        let importe = item.cantidad * item.precio_unitario;
        let detalle_id = Uuid::new_v4();

        let detalle_item = sqlx::query_as::<_, VentaDetalle>(
            r#"
            INSERT INTO ventas_detalle (id, venta_id, producto_id, cantidad,
                                        precio_unitario, importe)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, venta_id, producto_id, cantidad,
                      precio_unitario, importe, created_at
            "#,
        )
        .bind(detalle_id)
        .bind(venta_id)
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
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "El producto {} no tiene inventario en la sucursal {}",
                item.producto_id, dto.sucursal_id
            ))
        })?;

        sqlx::query(
            r#"
            INSERT INTO movimientos_stock (inventario_id, tipo_movimiento, cantidad,
                                           stock_antes, stock_despues, referencia_tipo,
                                           referencia_id, motivo, usuario_id, empresa_id)
            VALUES ($1, 'VENTA', $2, 0, 0, 'VENTA', $3, $4, $5, $6)
            "#,
        )
        .bind(inv.id)
        .bind(item.cantidad)
        .bind(venta_id)
        .bind(format!("Venta {}", folio))
        .bind(usuario_id)
        .bind(empresa_id)
        .execute(&mut *tx)
        .await?;

        detalle.push(detalle_item);
    }

    tx.commit().await?;

    Ok(VentaResponse { venta, detalle })
}

pub async fn listar_por_sucursal(
    pool: &PgPool,
    sucursal_id: Uuid,
    empresa_id: Uuid,
) -> Result<Vec<Venta>, AppError> {
    let rows = sqlx::query_as::<_, Venta>(
        r#"
        SELECT id, folio, cliente_id, sucursal_id, usuario_id,
               subtotal, iva, total, forma_pago, metodo_pago,
               uso_cfdi, estado, empresa_id, created_at, updated_at
        FROM ventas
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
) -> Result<VentaResponse, AppError> {
    let venta = sqlx::query_as::<_, Venta>(
        r#"
        SELECT id, folio, cliente_id, sucursal_id, usuario_id,
               subtotal, iva, total, forma_pago, metodo_pago,
               uso_cfdi, estado, empresa_id, created_at, updated_at
        FROM ventas
        WHERE id = $1 AND empresa_id = $2
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Venta no encontrada".to_string()))?;

    let detalle = sqlx::query_as::<_, VentaDetalle>(
        r#"
        SELECT id, venta_id, producto_id, cantidad,
               precio_unitario, importe, created_at
        FROM ventas_detalle
        WHERE venta_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    Ok(VentaResponse { venta, detalle })
}

pub async fn cancelar(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
) -> Result<VentaResponse, AppError> {
    let venta = sqlx::query_as::<_, Venta>(
        r#"
        UPDATE ventas
        SET estado = 'cancelada', updated_at = NOW()
        WHERE id = $1 AND empresa_id = $2 AND estado = 'completada'
        RETURNING id, folio, cliente_id, sucursal_id, usuario_id,
                  subtotal, iva, total, forma_pago, metodo_pago,
                  uso_cfdi, estado, empresa_id, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(
            "Venta no encontrada o ya fue cancelada".to_string(),
        )
    })?;

    let detalle = sqlx::query_as::<_, VentaDetalle>(
        r#"
        SELECT id, venta_id, producto_id, cantidad,
               precio_unitario, importe, created_at
        FROM ventas_detalle
        WHERE venta_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(id)
    .fetch_all(pool)
    .await?;

    let mut tx = pool.begin().await?;

    for item in &detalle {
        let inv = sqlx::query_as::<_, Inventario>(
            r#"
            SELECT id, producto_id, sucursal_id, stock_actual, stock_minimo,
                   stock_maximo, ubicacion, empresa_id, created_at, updated_at
            FROM inventario i
            JOIN ventas_detalle vd ON vd.producto_id = i.producto_id
            WHERE vd.id = $1 AND i.sucursal_id = $2 AND i.empresa_id = $3
            "#,
        )
        .bind(item.id)
        .bind(venta.sucursal_id)
        .bind(empresa_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| {
            AppError::NotFound(format!(
                "Inventario no encontrado para el producto {}",
                item.producto_id
            ))
        })?;

        sqlx::query(
            r#"
            INSERT INTO movimientos_stock (inventario_id, tipo_movimiento, cantidad,
                                           stock_antes, stock_despues, referencia_tipo,
                                           referencia_id, motivo, usuario_id, empresa_id)
            VALUES ($1, 'ENTRADA', $2, 0, 0, 'CANCELACION_VENTA', $3, $4, $5, $6)
            "#,
        )
        .bind(inv.id)
        .bind(item.cantidad)
        .bind(venta.id)
        .bind(format!("Cancelación venta {}", venta.folio))
        .bind(venta.usuario_id)
        .bind(empresa_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(VentaResponse { venta, detalle })
}
