-- ============================================================================
-- Crea el rol de aplicación app_user de forma idempotente.
-- Este rol es usado por el backend con RLS habilitado (NOBYPASSRLS).
-- Se ejecuta ANTES de cualquier migración que haga GRANT ... TO app_user.
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT FROM pg_catalog.pg_roles WHERE rolname = 'app_user'
    ) THEN
        CREATE ROLE app_user WITH LOGIN PASSWORD 'app_user' NOBYPASSRLS;
        RAISE NOTICE 'Rol app_user creado exitosamente';
    ELSE
        RAISE NOTICE 'Rol app_user ya existe, omitiendo creación';
    END IF;
END
$$;
