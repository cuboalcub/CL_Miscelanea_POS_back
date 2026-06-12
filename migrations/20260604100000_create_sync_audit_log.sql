-- ============================================================================
-- Auditoría de solicitudes de sincronización (POST /sync).
-- Cada fila resume un batch completo: qué dispositivo, cuántas operaciones,
-- cuántas exitosas/fallidas y el primer error para diagnóstico rápido.
--
-- Uso en soporte: consultar por dispositivo o sucursal ordenado por fecha
-- para identificar problemas de sincronización recurrentes.
-- ============================================================================

CREATE TABLE IF NOT EXISTS sync_audit_log (
    id                 UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dispositivo_id     UUID NOT NULL,
    sucursal_id        UUID NOT NULL REFERENCES sucursales(id) ON DELETE CASCADE,
    usuario_id         UUID REFERENCES usuarios(id) ON DELETE SET NULL,
    total_operaciones  INTEGER NOT NULL CHECK (total_operaciones >= 0),
    operaciones_ok     INTEGER NOT NULL CHECK (operaciones_ok >= 0),
    operaciones_error  INTEGER NOT NULL CHECK (operaciones_error >= 0),
    primer_error       TEXT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sync_audit_log_dispositivo
    ON sync_audit_log(dispositivo_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_sync_audit_log_sucursal
    ON sync_audit_log(sucursal_id, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_sync_audit_log_created
    ON sync_audit_log(created_at DESC);
