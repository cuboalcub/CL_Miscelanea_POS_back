-- Migración: admin de plataforma + dueño explícito de empresa
--
-- 1. Agrega columna owner_id a empresas (dueño explícito)
-- 2. Agrega columna es_admin_plataforma a usuarios (personal del SaaS)
-- 3. Seed del primer admin de plataforma

-- 1. Dueño explícito de cada empresa
ALTER TABLE empresas ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES usuarios(id);
CREATE INDEX IF NOT EXISTS idx_empresas_owner_id ON empresas(owner_id);

-- 2. Flag de admin de plataforma (personal del SaaS, ve todas las empresas)
ALTER TABLE usuarios ADD COLUMN IF NOT EXISTS es_admin_plataforma BOOLEAN NOT NULL DEFAULT FALSE;

-- 3. Seed: primer admin de plataforma
-- Password por defecto: Admin123! (debe cambiarse en el primer inicio de sesión)
INSERT INTO usuarios (nombre, email, password_hash, es_admin_plataforma)
VALUES (
    'Super Admin Plataforma',
    'admin@plataforma.com',
    'HASH_PLACEHOLDER:Admin123!',
    TRUE
)
ON CONFLICT (email) DO NOTHING;
