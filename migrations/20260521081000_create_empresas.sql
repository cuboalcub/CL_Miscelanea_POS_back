-- Migración para crear la tabla de empresas (multi-tenancy)
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS empresas (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre VARCHAR(255) NOT NULL,
    rfc VARCHAR(13) NOT NULL,
    regimen_fiscal VARCHAR(100) NOT NULL,
    logo_url TEXT,
    subdominio VARCHAR(100) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Crear índice para búsquedas rápidas por subdominio (multi-tenancy)
CREATE INDEX IF NOT EXISTS idx_empresas_subdominio ON empresas(subdominio);

-- Trigger para actualizar el campo 'updated_at' automáticamente
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE OR REPLACE TRIGGER update_empresas_updated_at
    BEFORE UPDATE ON empresas
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
