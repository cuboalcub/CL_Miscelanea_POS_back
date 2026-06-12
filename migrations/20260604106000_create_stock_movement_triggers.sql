-- ============================================================================
-- Triggers para actualizar inventario automáticamente al registrar movimientos
-- ============================================================================

CREATE OR REPLACE FUNCTION fn_movimiento_stock_before()
RETURNS TRIGGER AS $$
DECLARE
    v_actual DOUBLE PRECISION;
BEGIN
    -- Bloqueo pesimista de la fila de inventario para evitar condiciones de carrera
    SELECT stock_actual INTO v_actual
    FROM inventario
    WHERE id = NEW.inventario_id
    FOR UPDATE;

    IF NOT FOUND THEN
        RAISE EXCEPTION 'Inventario con id % no encontrado', NEW.inventario_id
            USING ERRCODE = 'P0002';
    END IF;

    -- Registrar el stock antes del movimiento
    NEW.stock_antes := v_actual;

    -- Calcular stock_despues según el tipo de movimiento
    -- ENTRADA:  stock_actual + cantidad
    -- SALIDA / VENTA / TRASPASO:  stock_actual - cantidad
    -- AJUSTE:   cantidad es el valor absoluto objetivo (ajuste por conteo físico)
    IF NEW.tipo_movimiento = 'ENTRADA' THEN
        NEW.stock_despues := v_actual + NEW.cantidad;
    ELSIF NEW.tipo_movimiento IN ('SALIDA', 'VENTA', 'TRASPASO') THEN
        NEW.stock_despues := v_actual - NEW.cantidad;
    ELSIF NEW.tipo_movimiento = 'AJUSTE' THEN
        NEW.stock_despues := NEW.cantidad;
    END IF;

    -- Validar que el stock resultante no sea negativo
    IF NEW.stock_despues < 0 THEN
        RAISE EXCEPTION 'Stock insuficiente en inventario %: disponible %, requerido %',
            NEW.inventario_id, v_actual, v_actual - NEW.stock_despues
            USING ERRCODE = 'P0002';
    END IF;

    -- Actualizar la tabla de inventario
    UPDATE inventario
    SET stock_actual = NEW.stock_despues
    WHERE id = NEW.inventario_id;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_movimiento_stock_before
    BEFORE INSERT ON movimientos_stock
    FOR EACH ROW
    EXECUTE FUNCTION fn_movimiento_stock_before();
