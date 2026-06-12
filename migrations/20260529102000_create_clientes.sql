-- ============================================================================
-- Tabla de clientes — sincronizable desde dispositivos móviles
-- ============================================================================

CREATE TABLE IF NOT EXISTS clientes (
    id              UUID PRIMARY KEY,
    nombre          VARCHAR(255) NOT NULL,
    rfc             VARCHAR(13),
    email           VARCHAR(255),
    telefono        VARCHAR(20),
    direccion       TEXT,
    regimen_fiscal  VARCHAR(10),
    activo          BOOLEAN NOT NULL DEFAULT TRUE,
    empresa_id      UUID NOT NULL REFERENCES empresas(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_clientes_empresa ON clientes(empresa_id);
CREATE INDEX IF NOT EXISTS idx_clientes_nombre ON clientes(nombre);
