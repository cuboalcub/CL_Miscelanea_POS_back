-- Migración: enum de nivel de acceso y tabla pivot 'perfiles'
--
-- 'perfiles' vincula un usuario con UNA empresa y OPCIONALMENTE una sucursal.
-- Si sucursal_id es NULL → el perfil aplica a toda la empresa.
-- Si sucursal_id tiene valor → el perfil aplica solo a esa sucursal.
--
-- Niveles de acceso (de mayor a menor privilegio):
--   super_admin  → acceso total al sistema (gestión de empresas, usuarios, config)
--   admin        → gestión completa dentro de su empresa/sucursal
--   cajero       → operaciones de venta y caja
--   consulta     → solo lectura de reportes y catálogos

CREATE TYPE nivel_acceso AS ENUM (
    'super_admin',
    'admin',
    'cajero',
    'consulta'
);

CREATE TABLE IF NOT EXISTS perfiles (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    usuario_id   UUID NOT NULL,
    empresa_id   UUID NOT NULL,
    -- NULL = acceso a toda la empresa; NOT NULL = solo esa sucursal
    sucursal_id  UUID,
    nivel        nivel_acceso NOT NULL DEFAULT 'consulta',
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- FK → usuarios
    CONSTRAINT fk_perfiles_usuario
        FOREIGN KEY (usuario_id) REFERENCES usuarios(id) ON DELETE CASCADE,

    -- FK → empresas
    CONSTRAINT fk_perfiles_empresa
        FOREIGN KEY (empresa_id) REFERENCES empresas(id) ON DELETE CASCADE,

    -- FK → sucursales (nullable)
    CONSTRAINT fk_perfiles_sucursal
        FOREIGN KEY (sucursal_id) REFERENCES sucursales(id) ON DELETE CASCADE,

    -- Un usuario no puede tener dos perfiles con el mismo scope (empresa+sucursal)
    CONSTRAINT uq_perfil_scope
        UNIQUE (usuario_id, empresa_id, sucursal_id)
);

-- Consultas frecuentes: "¿qué perfiles tiene este usuario?"
CREATE INDEX IF NOT EXISTS idx_perfiles_usuario_id  ON perfiles(usuario_id);
-- "¿quiénes tienen acceso a esta empresa?"
CREATE INDEX IF NOT EXISTS idx_perfiles_empresa_id  ON perfiles(empresa_id);
-- "¿quiénes tienen acceso a esta sucursal?"
CREATE INDEX IF NOT EXISTS idx_perfiles_sucursal_id ON perfiles(sucursal_id) WHERE sucursal_id IS NOT NULL;

CREATE OR REPLACE TRIGGER update_perfiles_updated_at
    BEFORE UPDATE ON perfiles
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
