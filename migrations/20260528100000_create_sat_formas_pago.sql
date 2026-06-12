-- Migración para crear el catálogo oficial del SAT de Formas de Pago (c_FormaPago) para CFDI 4.0.
-- Define la tabla de almacenamiento y precarga de formas de pago estándar.

-- 1. Crear la tabla de formas de pago del SAT
CREATE TABLE IF NOT EXISTS sat_formas_pago (
    clave        VARCHAR(10) PRIMARY KEY, -- Clave numérica de 2 dígitos del SAT
    descripcion  VARCHAR(255) NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Crear índice para optimizar búsquedas por descripción
CREATE INDEX IF NOT EXISTS idx_sat_formas_pago_descripcion ON sat_formas_pago(descripcion);

-- 3. Precargar catálogo con formas de pago oficiales del SAT (c_FormaPago)
INSERT INTO sat_formas_pago (clave, descripcion) VALUES
('01', 'Efectivo'),
('02', 'Cheque nominativo'),
('03', 'Transferencia electrónica de fondos'),
('04', 'Tarjeta de crédito'),
('05', 'Monedero electrónico'),
('06', 'Dinero electrónico'),
('08', 'Vales de despensa'),
('12', 'Dación en pago'),
('13', 'Pago por subrogación'),
('14', 'Pago por consignación'),
('15', 'Condonación'),
('17', 'Compensación'),
('23', 'Novación'),
('24', 'Confusión'),
('25', 'Remisión de deuda'),
('26', 'Prescripción o caducidad'),
('27', 'A satisfacción del acreedor'),
('28', 'Tarjeta de débito'),
('29', 'Tarjeta de servicio'),
('30', 'Aplicación de anticipos'),
('31', 'Intermediario pagos'),
('99', 'Por definir')
ON CONFLICT (clave) DO UPDATE SET 
    descripcion = EXCLUDED.descripcion;
