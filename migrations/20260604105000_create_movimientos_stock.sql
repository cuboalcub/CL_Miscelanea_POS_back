-- ============================================================================
-- Tabla histórica de movimientos de stock (auditoría de inventario)
-- ============================================================================

CREATE TABLE IF NOT EXISTS movimientos_stock (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    inventario_id    UUID NOT NULL REFERENCES inventario(id) ON DELETE CASCADE,
    tipo_movimiento  VARCHAR(20) NOT NULL
        CHECK (tipo_movimiento IN ('ENTRADA', 'SALIDA', 'AJUSTE', 'VENTA', 'TRASPASO')),
    cantidad         DOUBLE PRECISION NOT NULL,
    stock_antes      DOUBLE PRECISION NOT NULL,
    stock_despues    DOUBLE PRECISION NOT NULL,
    referencia_tipo  VARCHAR(30),
    referencia_id    UUID,
    motivo           TEXT,
    usuario_id       UUID REFERENCES usuarios(id),
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_movimientos_stock_inventario
    ON movimientos_stock(inventario_id);
CREATE INDEX IF NOT EXISTS idx_movimientos_stock_fecha
    ON movimientos_stock(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_movimientos_stock_referencia
    ON movimientos_stock(referencia_tipo, referencia_id);
