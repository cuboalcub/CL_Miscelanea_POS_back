-- ============================================================================
-- Tabla de staging para operaciones recibidas desde dispositivos móviles
-- en modo offline. Cada fila representa una operación atómica del batch.
-- El par (dispositivo_id, cliente_id) garantiza idempotencia.
-- ============================================================================

CREATE TABLE IF NOT EXISTS sync_operations (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispositivo_id   UUID NOT NULL,
    sucursal_id      UUID NOT NULL REFERENCES sucursales(id) ON DELETE CASCADE,
    usuario_id       UUID REFERENCES usuarios(id) ON DELETE SET NULL,
    cliente_id       VARCHAR(64) NOT NULL,
    tipo_operacion   VARCHAR(50) NOT NULL,
    entidad          VARCHAR(50) NOT NULL,
    datos            JSONB NOT NULL,
    estado           VARCHAR(20) NOT NULL DEFAULT 'pendiente'
                         CHECK (estado IN ('pendiente', 'procesado', 'rechazado')),
    resultado        JSONB,
    ocurrido_en      TIMESTAMPTZ NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    procesado_en     TIMESTAMPTZ,

    UNIQUE (dispositivo_id, cliente_id)
);

CREATE INDEX IF NOT EXISTS idx_sync_operations_estado
    ON sync_operations(estado);

CREATE INDEX IF NOT EXISTS idx_sync_operations_dispositivo
    ON sync_operations(dispositivo_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_sync_operations_sucursal
    ON sync_operations(sucursal_id, created_at DESC);
