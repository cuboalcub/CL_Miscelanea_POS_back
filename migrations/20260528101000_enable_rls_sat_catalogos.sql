-- ============================================================================
-- Habilita Row Level Security en catálogos SAT y concede permisos al rol app_user
-- Los catálogos SAT son de solo lectura (SELECT) para todos los usuarios.
-- Los usuarios autenticados pueden leer, NADIE puede escribir vía la aplicación.
--
-- NOTA: Solo se incluyen las tablas que existen en este punto del timeline
-- de migraciones. Las tablas SAT adicionales habilitarán RLS en sus propias
-- migraciones de creación.
-- ============================================================================

-- ============================================================================
-- 1. Conceder permisos de esquema al rol de aplicación
-- ============================================================================
GRANT USAGE ON SCHEMA public TO app_user;

-- ============================================================================
-- 2. Conceder permisos sobre tablas existentes (SELECT general + CRUD en negocio)
-- ============================================================================
GRANT SELECT ON ALL TABLES IN SCHEMA public TO app_user;
GRANT INSERT, UPDATE, DELETE ON
    empresas,
    sucursales,
    usuarios,
    perfiles,
    perfiles_empleados,
    audit_logs
TO app_user;

-- ============================================================================
-- 3. Conceder uso de secuencias (para IDs seriales)
-- ============================================================================
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO app_user;

-- ============================================================================
-- 4. Habilitar RLS en tablas de catálogos SAT existentes
-- ============================================================================
ALTER TABLE sat_formas_pago      ENABLE ROW LEVEL SECURITY;
ALTER TABLE sat_claves_prod_serv ENABLE ROW LEVEL SECURITY;
ALTER TABLE sat_unidades_medida  ENABLE ROW LEVEL SECURITY;

-- ============================================================================
-- 5. Crear políticas: SELECT permitido para todos
--    Al no crear políticas FOR INSERT/UPDATE/DELETE, el default-deny de RLS
--    bloquea cualquier escritura desde el rol app_user.
-- ============================================================================
CREATE POLICY sat_select_all ON sat_formas_pago
    FOR SELECT USING (true);

CREATE POLICY sat_select_all ON sat_claves_prod_serv
    FOR SELECT USING (true);

CREATE POLICY sat_select_all ON sat_unidades_medida
    FOR SELECT USING (true);

-- ============================================================================
-- NOTA: El rol app_user NO es el owner de estas tablas (lo es postgres).
--       Al ser un rol regular (no superuser, no bypassrls), RLS se aplica
--       estrictamente. Las políticas FOR SELECT USING (true) permiten lectura
--       global, y la ausencia de políticas de escritura las bloquea.
-- ============================================================================
