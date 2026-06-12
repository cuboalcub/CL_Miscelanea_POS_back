-- ============================================================================
-- Crea el rol de aplicación con permisos limitados para RLS
-- Este script se ejecuta UNA SOLA VEZ al crear el contenedor PostgreSQL
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'app_user') THEN
        CREATE ROLE app_user WITH LOGIN PASSWORD 'app_user' NOBYPASSRLS;
        RAISE NOTICE 'Rol app_user creado exitosamente';
    ELSE
        RAISE NOTICE 'Rol app_user ya existe, omitiendo creación';
    END IF;
END
$$;
