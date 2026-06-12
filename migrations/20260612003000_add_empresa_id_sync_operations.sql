-- Agrega columna empresa_id a sync_operations y sync_audit_log

ALTER TABLE sync_operations
    ADD COLUMN empresa_id UUID NOT NULL REFERENCES empresas(id) DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE sync_operations
    ALTER COLUMN empresa_id DROP DEFAULT;

CREATE INDEX IF NOT EXISTS idx_sync_operations_empresa ON sync_operations(empresa_id);

ALTER TABLE sync_audit_log
    ADD COLUMN empresa_id UUID NOT NULL REFERENCES empresas(id) DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE sync_audit_log
    ALTER COLUMN empresa_id DROP DEFAULT;

CREATE INDEX IF NOT EXISTS idx_sync_audit_log_empresa ON sync_audit_log(empresa_id);

-- Habilitar RLS y aplicar políticas de aislamiento por tenant
ALTER TABLE sync_operations ENABLE ROW LEVEL SECURITY;
ALTER TABLE sync_audit_log  ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON sync_operations
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON sync_audit_log
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);
