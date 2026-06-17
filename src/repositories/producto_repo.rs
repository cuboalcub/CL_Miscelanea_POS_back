use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::{CreateProductoDto, Producto, UpdateProductoDto};

pub async fn listar(pool: &PgPool, empresa_id: Uuid) -> Result<Vec<Producto>, AppError> {
    let rows = sqlx::query_as::<_, Producto>(
        r#"
        SELECT id, sku, codigo_barras, nombre, descripcion, precio_venta, precio_compra,
               sat_clave, sat_unidad, categoria_id, iva_incluido, activo, empresa_id,
               created_at, updated_at
        FROM productos
        WHERE empresa_id = $1 AND activo = TRUE
        ORDER BY nombre ASC
        "#,
    )
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid, empresa_id: Uuid) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        SELECT id, sku, codigo_barras, nombre, descripcion, precio_venta, precio_compra,
               sat_clave, sat_unidad, categoria_id, iva_incluido, activo, empresa_id,
               created_at, updated_at
        FROM productos
        WHERE id = $1 AND empresa_id = $2 AND activo = TRUE
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Producto no encontrado".to_string()))
}

pub async fn crear(pool: &PgPool, dto: CreateProductoDto) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        INSERT INTO productos (id, sku, codigo_barras, nombre, descripcion, precio_venta,
                               precio_compra, sat_clave, sat_unidad, categoria_id,
                               iva_incluido, activo, empresa_id)
        VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, TRUE, $11)
        RETURNING *
        "#,
    )
    .bind(dto.sku)
    .bind(dto.codigo_barras)
    .bind(&dto.nombre)
    .bind(dto.descripcion)
    .bind(dto.precio_venta)
    .bind(dto.precio_compra)
    .bind(dto.sat_clave)
    .bind(dto.sat_unidad)
    .bind(dto.categoria_id)
    .bind(dto.iva_incluido.unwrap_or(true))
    .bind(dto.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn insertar(pool: &PgPool, producto: &Producto) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        INSERT INTO productos (id, sku, codigo_barras, nombre, descripcion, precio_venta,
                               precio_compra, sat_clave, sat_unidad, categoria_id,
                               iva_incluido, activo, empresa_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING *
        "#,
    )
    .bind(producto.id)
    .bind(&producto.sku)
    .bind(&producto.codigo_barras)
    .bind(&producto.nombre)
    .bind(&producto.descripcion)
    .bind(producto.precio_venta)
    .bind(producto.precio_compra)
    .bind(&producto.sat_clave)
    .bind(&producto.sat_unidad)
    .bind(producto.categoria_id)
    .bind(producto.iva_incluido)
    .bind(producto.activo)
    .bind(producto.empresa_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar(
    pool: &PgPool,
    id: Uuid,
    datos: &serde_json::Value,
) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        UPDATE productos SET
            sku            = COALESCE(($2)::VARCHAR, sku),
            codigo_barras  = COALESCE(($3)::VARCHAR, codigo_barras),
            nombre         = COALESCE(($4)::VARCHAR, nombre),
            descripcion    = COALESCE(($5)::VARCHAR, descripcion),
            precio_venta   = COALESCE(($6)::DOUBLE PRECISION, precio_venta),
            precio_compra  = COALESCE(($7)::DOUBLE PRECISION, precio_compra),
            sat_clave      = COALESCE(($8)::VARCHAR, sat_clave),
            sat_unidad     = COALESCE(($9)::VARCHAR, sat_unidad),
            categoria_id   = COALESCE(($10)::UUID, categoria_id),
            iva_incluido   = COALESCE(($11)::BOOLEAN, iva_incluido),
            activo         = COALESCE(($12)::BOOLEAN, activo),
            updated_at     = NOW()
        WHERE id = $1 AND empresa_id = $13
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(datos.get("sku").and_then(|v| v.as_str()))
    .bind(datos.get("codigo_barras").and_then(|v| v.as_str()))
    .bind(datos.get("nombre").and_then(|v| v.as_str()))
    .bind(datos.get("descripcion").and_then(|v| v.as_str()))
    .bind(datos.get("precio_venta").and_then(|v| v.as_f64()))
    .bind(datos.get("precio_compra").and_then(|v| v.as_f64()))
    .bind(datos.get("sat_clave").and_then(|v| v.as_str()))
    .bind(datos.get("sat_unidad").and_then(|v| v.as_str()))
    .bind(
        datos
            .get("categoria_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()),
    )
    .bind(datos.get("iva_incluido").and_then(|v| v.as_bool()))
    .bind(datos.get("activo").and_then(|v| v.as_bool()))
    .bind(
        datos
            .get("empresa_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok()),
    )
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Producto no encontrado".to_string()))
}

pub async fn actualizar_con_dto(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
    dto: UpdateProductoDto,
) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        UPDATE productos SET
            sku            = COALESCE($2, sku),
            codigo_barras  = COALESCE($3, codigo_barras),
            nombre         = COALESCE($4, nombre),
            descripcion    = COALESCE($5, descripcion),
            precio_venta   = COALESCE($6, precio_venta),
            precio_compra  = COALESCE($7, precio_compra),
            sat_clave      = COALESCE($8, sat_clave),
            sat_unidad     = COALESCE($9, sat_unidad),
            categoria_id   = COALESCE($10, categoria_id),
            iva_incluido   = COALESCE($11, iva_incluido),
            activo         = COALESCE($12, activo),
            updated_at     = NOW()
        WHERE id = $1 AND empresa_id = $13 AND activo = TRUE
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(dto.sku)
    .bind(dto.codigo_barras)
    .bind(dto.nombre)
    .bind(dto.descripcion)
    .bind(dto.precio_venta)
    .bind(dto.precio_compra)
    .bind(dto.sat_clave)
    .bind(dto.sat_unidad)
    .bind(dto.categoria_id)
    .bind(dto.iva_incluido)
    .bind(dto.activo)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Producto no encontrado".to_string()))
}

pub async fn eliminar(
    pool: &PgPool,
    id: Uuid,
    empresa_id: Uuid,
) -> Result<Producto, AppError> {
    let row = sqlx::query_as::<_, Producto>(
        r#"
        UPDATE productos SET activo = FALSE, updated_at = NOW()
        WHERE id = $1 AND empresa_id = $2 AND activo = TRUE
        RETURNING *
        "#,
    )
    .bind(id)
    .bind(empresa_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or_else(|| AppError::NotFound("Producto no encontrado".to_string()))
}
