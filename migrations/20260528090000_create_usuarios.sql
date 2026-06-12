-- Migración: tabla de usuarios del sistema
-- Almacena credenciales e identidad. El acceso a empresas/sucursales
-- se define en la tabla 'perfiles' (pivot).

CREATE TABLE IF NOT EXISTS usuarios (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre        VARCHAR(150) NOT NULL,
    email         VARCHAR(255) UNIQUE NOT NULL,
    -- hash bcrypt/argon2 de la contraseña; nunca texto plano
    password_hash TEXT NOT NULL,
    activo        BOOLEAN NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Búsquedas de login por email
CREATE UNIQUE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios(email);

CREATE OR REPLACE TRIGGER update_usuarios_updated_at
    BEFORE UPDATE ON usuarios
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
