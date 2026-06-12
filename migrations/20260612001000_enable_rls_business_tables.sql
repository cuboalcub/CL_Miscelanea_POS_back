-- Habilita RLS en todas las tablas de negocio y crea políticas de aislamiento
-- por tenant usando la variable de sesión app.current_empresa_id.

-- 1. Habilitar RLS
ALTER TABLE empresas              ENABLE ROW LEVEL SECURITY;
ALTER TABLE sucursales            ENABLE ROW LEVEL SECURITY;
ALTER TABLE usuarios              ENABLE ROW LEVEL SECURITY;
ALTER TABLE perfiles              ENABLE ROW LEVEL SECURITY;
ALTER TABLE perfiles_empleados    ENABLE ROW LEVEL SECURITY;
ALTER TABLE productos             ENABLE ROW LEVEL SECURITY;
ALTER TABLE clientes              ENABLE ROW LEVEL SECURITY;
ALTER TABLE categorias            ENABLE ROW LEVEL SECURITY;
-- NOTE: sync_operations and sync_audit_log get ENABLE ROW LEVEL SECURITY + policies
-- in 20260612003000_add_empresa_id_sync_operations.sql, after empresa_id is added.

-- 2. Políticas para tablas con empresa_id directo
CREATE POLICY tenant_isolation ON empresas
    FOR ALL USING (id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON sucursales
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON perfiles
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON perfiles_empleados
    FOR ALL USING (true);

CREATE POLICY tenant_isolation ON productos
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON clientes
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON categorias
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);


-- 3. Política para usuarios: todos pueden ver usuarios (son globales),
--    pero solo el propio usuario o admin de plataforma puede modificarlos
CREATE POLICY tenant_isolation_select ON usuarios
    FOR SELECT USING (true);

CREATE POLICY tenant_isolation_write ON usuarios
    FOR INSERT WITH CHECK (true);

CREATE POLICY tenant_isolation_update ON usuarios
    FOR UPDATE USING (
        id = current_setting('app.current_usuario_id')::UUID
        OR (SELECT es_admin_plataforma FROM usuarios WHERE id = current_setting('app.current_usuario_id')::UUID)
    );

CREATE POLICY tenant_isolation_delete ON usuarios
    FOR DELETE USING (
        id = current_setting('app.current_usuario_id')::UUID
        OR (SELECT es_admin_plataforma FROM usuarios WHERE id = current_setting('app.current_usuario_id')::UUID)
    );
