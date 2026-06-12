-- Migración para asegurar la existencia de índices B-Tree en la columna 'empresa_id'
-- de todas las tablas principales, optimizando el filtrado por tenant (multi-tenancy).

-- Índice B-Tree para la tabla sucursales
CREATE INDEX IF NOT EXISTS idx_sucursales_empresa_id ON sucursales USING btree (empresa_id);

-- Índice B-Tree para la tabla perfiles (pivot usuario-empresa-sucursal)
CREATE INDEX IF NOT EXISTS idx_perfiles_empresa_id ON perfiles USING btree (empresa_id);
