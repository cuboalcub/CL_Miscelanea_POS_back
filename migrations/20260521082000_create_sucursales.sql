-- Migración para crear la tabla de sucursales vinculada a empresas
CREATE TABLE IF NOT EXISTS sucursales (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    nombre VARCHAR(150) NOT NULL,
    direccion_completa TEXT NOT NULL,
    telefono VARCHAR(20),
    encargado VARCHAR(150),
    empresa_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    -- Restricción de clave foránea vinculada a empresas
    CONSTRAINT fk_sucursales_empresa
        FOREIGN KEY (empresa_id)
        REFERENCES empresas(id)
        ON DELETE CASCADE
);

-- Crear índice para búsquedas rápidas de sucursales por empresa
CREATE INDEX IF NOT EXISTS idx_sucursales_empresa_id ON sucursales(empresa_id);

-- Trigger para actualizar el campo 'updated_at' automáticamente
CREATE OR REPLACE TRIGGER update_sucursales_updated_at
    BEFORE UPDATE ON sucursales
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
