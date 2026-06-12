-- Migración para crear la tabla de auditoría (audit_logs)
-- y la infraestructura de triggers genéricos para registrar cambios críticos.

-- 1. Crear la tabla de logs de auditoría
CREATE TABLE IF NOT EXISTS audit_logs (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tabla_nombre   VARCHAR(100) NOT NULL,
    operacion      VARCHAR(10) NOT NULL,              -- 'INSERT', 'UPDATE', 'DELETE'
    valores_previos JSONB,                            -- NULL en un INSERT
    valores_nuevos  JSONB,                            -- NULL en un DELETE
    usuario_id     UUID,                              -- Obtenido del contexto de sesión app.current_usuario_id
    empresa_id     UUID,                              -- Multi-tenant: Obtenido del contexto o del registro
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Índices B-Tree para optimizar consultas de auditoría frecuentes
CREATE INDEX IF NOT EXISTS idx_audit_logs_tabla ON audit_logs(tabla_nombre);
CREATE INDEX IF NOT EXISTS idx_audit_logs_usuario ON audit_logs(usuario_id) WHERE usuario_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_logs_empresa ON audit_logs(empresa_id) WHERE empresa_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_audit_logs_created ON audit_logs(created_at DESC);

-- 2. Función del trigger genérico de auditoría
CREATE OR REPLACE FUNCTION process_audit_log()
RETURNS TRIGGER AS $$
DECLARE
    session_user_id TEXT;
    session_tenant_id TEXT;
    v_valores_previos JSONB := NULL;
    v_valores_nuevos JSONB := NULL;
    v_empresa_id UUID := NULL;
BEGIN
    -- Obtener información de usuario y tenant desde el contexto de la transacción/sesión
    session_user_id := current_setting('app.current_usuario_id', true);
    session_tenant_id := current_setting('app.current_empresa_id', true);

    -- Configurar valores según el tipo de operación
    IF (TG_OP = 'INSERT') THEN
        v_valores_nuevos := to_jsonb(NEW);
        
        -- Intentar obtener la empresa del registro si tiene dicha columna
        BEGIN
            v_empresa_id := NEW.empresa_id;
        EXCEPTION WHEN OTHERS THEN
            v_empresa_id := NULL;
        END;
        
    ELSIF (TG_OP = 'UPDATE') THEN
        v_valores_previos := to_jsonb(OLD);
        v_valores_nuevos := to_jsonb(NEW);
        
        BEGIN
            v_empresa_id := NEW.empresa_id;
        EXCEPTION WHEN OTHERS THEN
            v_empresa_id := NULL;
        END;
        
    ELSIF (TG_OP = 'DELETE') THEN
        v_valores_previos := to_jsonb(OLD);
        
        BEGIN
            v_empresa_id := OLD.empresa_id;
        EXCEPTION WHEN OTHERS THEN
            v_empresa_id := NULL;
        END;
    END IF;

    -- Si el tenant de sesión está especificado, tiene prioridad sobre el del registro
    IF session_tenant_id IS NOT NULL AND session_tenant_id <> '' THEN
        v_empresa_id := session_tenant_id::UUID;
    END IF;

    -- Insertar el registro en la tabla de logs
    INSERT INTO audit_logs (
        tabla_nombre,
        operacion,
        valores_previos,
        valores_nuevos,
        usuario_id,
        empresa_id
    ) VALUES (
        TG_TABLE_NAME,
        TG_OP,
        v_valores_previos,
        v_valores_nuevos,
        NULLIF(session_user_id, '')::UUID,
        v_empresa_id
    );

    -- Retornar apropiadamente según la operación
    IF (TG_OP = 'DELETE') THEN
        RETURN OLD;
    ELSE
        RETURN NEW;
    END IF;
END;
$$ LANGUAGE plpgsql;

-- 3. Vincular el trigger a las tablas críticas
-- Empresas
CREATE OR REPLACE TRIGGER trigger_audit_empresas
    AFTER INSERT OR UPDATE OR DELETE ON empresas
    FOR EACH ROW
    EXECUTE FUNCTION process_audit_log();

-- Sucursales
CREATE OR REPLACE TRIGGER trigger_audit_sucursales
    AFTER INSERT OR UPDATE OR DELETE ON sucursales
    FOR EACH ROW
    EXECUTE FUNCTION process_audit_log();

-- Usuarios
CREATE OR REPLACE TRIGGER trigger_audit_usuarios
    AFTER INSERT OR UPDATE OR DELETE ON usuarios
    FOR EACH ROW
    EXECUTE FUNCTION process_audit_log();

-- Perfiles
CREATE OR REPLACE TRIGGER trigger_audit_perfiles
    AFTER INSERT OR UPDATE OR DELETE ON perfiles
    FOR EACH ROW
    EXECUTE FUNCTION process_audit_log();
