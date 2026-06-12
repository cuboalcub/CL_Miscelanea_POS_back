-- Agrega columna empresa_id a inventario y movimientos_stock
-- para permitir aislamiento por tenant vía RLS.

ALTER TABLE inventario
    ADD COLUMN empresa_id UUID NOT NULL REFERENCES empresas(id) DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE inventario
    ALTER COLUMN empresa_id DROP DEFAULT;

CREATE INDEX IF NOT EXISTS idx_inventario_empresa ON inventario(empresa_id);

ALTER TABLE movimientos_stock
    ADD COLUMN empresa_id UUID NOT NULL REFERENCES empresas(id) DEFAULT '00000000-0000-0000-0000-000000000000';

ALTER TABLE movimientos_stock
    ALTER COLUMN empresa_id DROP DEFAULT;

CREATE INDEX IF NOT EXISTS idx_movimientos_stock_empresa ON movimientos_stock(empresa_id);

ALTER TABLE inventario ENABLE ROW LEVEL SECURITY;
ALTER TABLE movimientos_stock ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON inventario
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);

CREATE POLICY tenant_isolation ON movimientos_stock
    FOR ALL USING (empresa_id = current_setting('app.current_empresa_id')::UUID);
