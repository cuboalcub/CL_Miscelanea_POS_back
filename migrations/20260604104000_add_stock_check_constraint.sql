-- ============================================================================
-- CHECK constraint para prevenir existencias negativas en inventario
-- ============================================================================

ALTER TABLE inventario
    ADD CONSTRAINT ck_inventario_stock_actual_positivo
    CHECK (stock_actual >= 0);

ALTER TABLE inventario
    ADD CONSTRAINT ck_inventario_stock_maximo_positivo
    CHECK (stock_maximo >= 0);

ALTER TABLE inventario
    ADD CONSTRAINT ck_inventario_stock_minimo_positivo
    CHECK (stock_minimo >= 0);
