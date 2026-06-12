-- ============================================================================
-- Tabla de inventario (existencias) por sucursal
-- ============================================================================

CREATE TABLE IF NOT EXISTS inventario (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    producto_id  UUID NOT NULL REFERENCES productos(id) ON DELETE CASCADE,
    sucursal_id  UUID NOT NULL REFERENCES sucursales(id) ON DELETE CASCADE,
    stock_actual DOUBLE PRECISION NOT NULL DEFAULT 0,
    stock_minimo DOUBLE PRECISION NOT NULL DEFAULT 0,
    stock_maximo DOUBLE PRECISION NOT NULL DEFAULT 0,
    ubicacion    VARCHAR(100),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_inventario_producto_sucursal UNIQUE (producto_id, sucursal_id)
);

CREATE INDEX IF NOT EXISTS idx_inventario_sucursal  ON inventario(sucursal_id);
CREATE INDEX IF NOT EXISTS idx_inventario_producto  ON inventario(producto_id);

CREATE TRIGGER update_inventario_updated_at
    BEFORE UPDATE ON inventario
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
