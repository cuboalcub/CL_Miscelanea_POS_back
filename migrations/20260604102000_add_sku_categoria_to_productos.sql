-- ============================================================================
-- Agrega SKU y relación con categorías a la tabla de productos
-- ============================================================================

ALTER TABLE productos
    ADD COLUMN IF NOT EXISTS sku         VARCHAR(50),
    ADD COLUMN IF NOT EXISTS categoria_id UUID REFERENCES categorias(id) ON DELETE SET NULL;

CREATE INDEX IF NOT EXISTS idx_productos_sku       ON productos(empresa_id, sku);
CREATE INDEX IF NOT EXISTS idx_productos_categoria  ON productos(categoria_id);

-- SKU único por empresa (NULLs no compiten en el índice parcial)
CREATE UNIQUE INDEX IF NOT EXISTS idx_productos_sku_unique
    ON productos(empresa_id, sku)
    WHERE sku IS NOT NULL;

-- Trigger para updated_at (safe: IF NOT EXISTS para triggers no existe,
-- así que usamos DO para hacerlo idempotente)
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_trigger
        WHERE tgname = 'update_productos_updated_at'
          AND tgrelid = 'productos'::regclass
    ) THEN
        CREATE TRIGGER update_productos_updated_at
            BEFORE UPDATE ON productos
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column();
    END IF;
END
$$;
