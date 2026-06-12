-- ============================================================================
-- Tabla de productos — sincronizable desde dispositivos móviles
-- ============================================================================

CREATE TABLE IF NOT EXISTS productos (
    id              UUID PRIMARY KEY,
    codigo_barras   VARCHAR(50),
    nombre          VARCHAR(255) NOT NULL,
    descripcion     TEXT,
    precio_venta    DOUBLE PRECISION NOT NULL DEFAULT 0,
    precio_compra   DOUBLE PRECISION NOT NULL DEFAULT 0,
    sat_clave       VARCHAR(8),
    sat_unidad      VARCHAR(10),
    iva_incluido    BOOLEAN NOT NULL DEFAULT TRUE,
    activo          BOOLEAN NOT NULL DEFAULT TRUE,
    empresa_id      UUID NOT NULL REFERENCES empresas(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_productos_empresa ON productos(empresa_id);
CREATE INDEX IF NOT EXISTS idx_productos_nombre ON productos(nombre);
CREATE INDEX IF NOT EXISTS idx_productos_codigo ON productos(codigo_barras);
