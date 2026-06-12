-- Migración para crear la función y triggers que asignan automáticamente
-- el 'empresa_id' basándose en la configuración de la sesión actual de PostgreSQL.
--
-- Uso en el Backend (Rust):
-- Antes de ejecutar una inserción, se ejecuta en la misma transacción:
--   SET LOCAL app.current_empresa_id = 'uuid-de-la-empresa';
-- Y el trigger se encargará de asignar automáticamente el valor si viene NULL.

-- 1. Crear o reemplazar la función del trigger
CREATE OR REPLACE FUNCTION assign_tenant_from_session()
RETURNS TRIGGER AS $$
DECLARE
    session_tenant_id TEXT;
BEGIN
    -- Obtener la variable de sesión configurada para el tenant actual
    -- El segundo parámetro 'true' previene excepciones si la variable no está definida, retornando NULL
    session_tenant_id := current_setting('app.current_empresa_id', true);
    
    -- Si el empresa_id no fue proveído explícitamente en el INSERT, y la variable de sesión tiene un valor válido
    IF NEW.empresa_id IS NULL AND session_tenant_id IS NOT NULL AND session_tenant_id <> '' THEN
        NEW.empresa_id := session_tenant_id::UUID;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 2. Vincular el trigger a la tabla 'sucursales'
CREATE OR REPLACE TRIGGER trigger_assign_tenant_sucursales
    BEFORE INSERT ON sucursales
    FOR EACH ROW
    EXECUTE FUNCTION assign_tenant_from_session();

-- 3. Vincular el trigger a la tabla 'perfiles'
CREATE OR REPLACE TRIGGER trigger_assign_tenant_perfiles
    BEFORE INSERT ON perfiles
    FOR EACH ROW
    EXECUTE FUNCTION assign_tenant_from_session();
