-- Migración para sincronizar automáticamente los roles del usuario (perfiles)
-- dentro de la metadata de autenticación nativa (auth.users.raw_app_meta_data).
--
-- Esto permite que los roles se incluyan de forma segura en el JWT emitido por Supabase Auth,
-- facilitando el control de acceso en políticas RLS (Row Level Security).

-- 1. Asegurar que auth.users posea la columna raw_app_meta_data (por compatibilidad local)
ALTER TABLE auth.users ADD COLUMN IF NOT EXISTS raw_app_meta_data JSONB DEFAULT '{}'::JSONB;

-- 2. Función del trigger para sincronizar perfiles activos a la app_metadata del usuario
CREATE OR REPLACE FUNCTION public.sync_user_roles_to_metadata()
RETURNS TRIGGER AS $$
DECLARE
    v_roles JSONB;
    v_usuario_id UUID;
BEGIN
    -- Obtener el ID del usuario del registro afectado (NEW para INSERT/UPDATE, OLD para DELETE)
    v_usuario_id := COALESCE(NEW.usuario_id, OLD.usuario_id);

    -- Agrupar todos los scopes de rol y nivel activos del empleado
    SELECT COALESCE(
        jsonb_agg(
            jsonb_build_object(
                'empresa_id', empresa_id,
                'sucursal_id', sucursal_id,
                'nivel', nivel
            )
        ),
        '[]'::JSONB
    ) INTO v_roles
    FROM public.perfiles
    WHERE usuario_id = v_usuario_id AND activo = true;

    -- Actualizar raw_app_meta_data en auth.users del esquema nativo
    UPDATE auth.users
    SET raw_app_meta_data = jsonb_set(
        COALESCE(raw_app_meta_data, '{}'::JSONB),
        '{roles}',
        v_roles
    )
    WHERE id = v_usuario_id;

    RETURN COALESCE(NEW, OLD);
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- 3. Crear el trigger para ejecutar tras INSERT, UPDATE o DELETE de perfiles de acceso
DROP TRIGGER IF EXISTS trigger_sync_user_roles ON public.perfiles;

CREATE TRIGGER trigger_sync_user_roles
    AFTER INSERT OR UPDATE OR DELETE ON public.perfiles
    FOR EACH ROW
    EXECUTE FUNCTION public.sync_user_roles_to_metadata();

-- 4. Ejecutar sincronización inicial para perfiles ya existentes (si los hay)
DO $$
DECLARE
    r RECORD;
BEGIN
    FOR r IN SELECT DISTINCT usuario_id FROM public.perfiles LOOP
        PERFORM public.sync_user_roles_to_metadata();
    END LOOP;
END;
$$;
