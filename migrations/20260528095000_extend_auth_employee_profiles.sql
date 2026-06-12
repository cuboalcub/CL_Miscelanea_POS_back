-- Migración para extender el esquema de autenticación nativo (Supabase Auth / auth.users)
-- con una tabla de perfiles de empleados ('perfiles_empleados') para datos adicionales.

-- 1. Crear el esquema 'auth' si no existe (facilita pruebas y desarrollo local si no está Supabase)
CREATE SCHEMA IF NOT EXISTS auth;

-- 2. Crear una tabla simulada de auth.users si no existe para asegurar la compatibilidad local
CREATE TABLE IF NOT EXISTS auth.users (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email               VARCHAR(255) UNIQUE,
    raw_user_meta_data  JSONB, -- Simula metadatos provistos por Supabase (nombre, teléfono, etc.)
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Crear la tabla de datos adicionales del empleado ('perfiles_empleados')
-- vinculada directamente por ID de relación 1:1 con el auth nativo.
CREATE TABLE IF NOT EXISTS perfiles_empleados (
    id                 UUID PRIMARY KEY, -- Mismo ID que auth.users
    nombre_completo    VARCHAR(255) NOT NULL,
    telefono           VARCHAR(20),
    puesto             VARCHAR(100),
    curp               VARCHAR(18) UNIQUE,
    nss                VARCHAR(11) UNIQUE,
    fecha_nacimiento   DATE,
    fecha_ingreso      DATE NOT NULL DEFAULT CURRENT_DATE,
    direccion          TEXT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Relación e integridad referencial 1:1 con la autenticación nativa
    CONSTRAINT fk_perfiles_empleados_auth
        FOREIGN KEY (id) REFERENCES auth.users(id) ON DELETE CASCADE
);

-- Índices B-Tree para optimizar búsquedas por CURP o NSS de empleados
CREATE UNIQUE INDEX IF NOT EXISTS idx_perfiles_empleados_curp ON perfiles_empleados(curp) WHERE curp IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_perfiles_empleados_nss ON perfiles_empleados(nss) WHERE nss IS NOT NULL;

-- Trigger para actualizar updated_at automáticamente
CREATE OR REPLACE TRIGGER update_perfiles_empleados_updated_at
    BEFORE UPDATE ON perfiles_empleados
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- 4. Función y Trigger automáticos en auth.users
-- Crea el perfil del empleado inmediatamente después de que el usuario se registra/crea en Supabase Auth
CREATE OR REPLACE FUNCTION public.handle_new_auth_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.perfiles_empleados (
        id,
        nombre_completo,
        telefono
    ) VALUES (
        NEW.id,
        COALESCE(NEW.raw_user_meta_data->>'nombre_completo', SPLIT_PART(NEW.email, '@', 1)),
        NEW.raw_user_meta_data->>'telefono'
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Eliminar trigger existente si lo hay para evitar duplicados en actualizaciones de migración
DROP TRIGGER IF EXISTS trigger_on_auth_user_created ON auth.users;

CREATE TRIGGER trigger_on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW
    EXECUTE FUNCTION public.handle_new_auth_user();

-- 5. Vincular al sistema de auditoría creado anteriormente
CREATE OR REPLACE TRIGGER trigger_audit_perfiles_empleados
    AFTER INSERT OR UPDATE OR DELETE ON perfiles_empleados
    FOR EACH ROW
    EXECUTE FUNCTION process_audit_log();
