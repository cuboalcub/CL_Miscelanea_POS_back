use sqlx::PgPool;

use crate::errors::AppError;
use crate::models::{
    SatClaveProdServ, SatExportacion, SatFormaPago, SatMetodoPago, SatRegimenFiscal,
    SatTipoComprobante, SatUsoCfdi,
};

/// Busca dentro del catálogo oficial del SAT por clave o descripción.
///
/// Implementa ordenamiento inteligente (priorizando coincidencias exactas de clave)
/// y paginación implícita de 50 registros para optimizar el rendimiento.
pub async fn buscar(pool: &PgPool, query: &str) -> Result<Vec<SatClaveProdServ>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        // Carga inicial o UX vacía: devolver los primeros 20 registros activos ordenados por clave
        let rows = sqlx::query_as::<_, SatClaveProdServ>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_claves_prod_serv
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    // Patrón de coincidencia ILIKE para descripción
    let search_pattern = format!("%{}%", clean_query);
    // Prefijo de coincidencia LIKE para clave
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatClaveProdServ>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_claves_prod_serv
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca formas de pago dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_formas_pago(pool: &PgPool, query: &str) -> Result<Vec<SatFormaPago>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatFormaPago>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_formas_pago
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatFormaPago>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_formas_pago
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca métodos de pago dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_metodos_pago(pool: &PgPool, query: &str) -> Result<Vec<SatMetodoPago>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatMetodoPago>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_metodos_pago
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatMetodoPago>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_metodos_pago
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca usos del CFDI dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_usos_cfdi(pool: &PgPool, query: &str) -> Result<Vec<SatUsoCfdi>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatUsoCfdi>(
            r#"
            SELECT clave, descripcion, regimen_fiscal_aplica, activo, created_at
            FROM sat_usos_cfdi
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatUsoCfdi>(
        r#"
        SELECT clave, descripcion, regimen_fiscal_aplica, activo, created_at
        FROM sat_usos_cfdi
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca regímenes fiscales dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_regimenes_fiscales(pool: &PgPool, query: &str) -> Result<Vec<SatRegimenFiscal>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatRegimenFiscal>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_regimenes_fiscales
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatRegimenFiscal>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_regimenes_fiscales
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca tipos de comprobante dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_tipos_comprobante(pool: &PgPool, query: &str) -> Result<Vec<SatTipoComprobante>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatTipoComprobante>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_tipos_comprobante
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatTipoComprobante>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_tipos_comprobante
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca claves de exportación dentro del catálogo oficial del SAT por clave o descripción.
pub async fn buscar_exportacion(pool: &PgPool, query: &str) -> Result<Vec<SatExportacion>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, SatExportacion>(
            r#"
            SELECT clave, descripcion, activo, created_at
            FROM sat_exportacion
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, SatExportacion>(
        r#"
        SELECT clave, descripcion, activo, created_at
        FROM sat_exportacion
        WHERE (clave LIKE $1 OR descripcion ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

/// Busca unidades de medida dentro del catálogo oficial del SAT por clave o nombre.
pub async fn buscar_unidades(pool: &PgPool, query: &str) -> Result<Vec<crate::models::SatUnidadMedida>, AppError> {
    let clean_query = query.trim();

    if clean_query.is_empty() {
        let rows = sqlx::query_as::<_, crate::models::SatUnidadMedida>(
            r#"
            SELECT clave, nombre, descripcion, activo, created_at
            FROM sat_unidades_medida
            WHERE activo = true
            ORDER BY clave ASC
            LIMIT 20
            "#,
        )
        .fetch_all(pool)
        .await?;
        return Ok(rows);
    }

    let search_pattern = format!("%{}%", clean_query);
    let key_prefix = format!("{}%", clean_query);

    let rows = sqlx::query_as::<_, crate::models::SatUnidadMedida>(
        r#"
        SELECT clave, nombre, descripcion, activo, created_at
        FROM sat_unidades_medida
        WHERE (clave LIKE $1 OR nombre ILIKE $2) AND activo = true
        ORDER BY 
            CASE 
                WHEN clave = $3 THEN 0
                WHEN clave LIKE $1 THEN 1
                ELSE 2
            END,
            clave ASC
        LIMIT 50
        "#,
    )
    .bind(&key_prefix)
    .bind(&search_pattern)
    .bind(clean_query)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

