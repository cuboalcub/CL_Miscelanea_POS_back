-- Migración para crear el catálogo oficial del SAT de Métodos de Pago (c_MetodoPago) para CFDI 4.0.
CREATE TABLE IF NOT EXISTS sat_metodos_pago (
    clave        VARCHAR(10) PRIMARY KEY,
    descripcion  VARCHAR(255) NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sat_metodos_pago_descripcion ON sat_metodos_pago(descripcion);

INSERT INTO sat_metodos_pago (clave, descripcion) VALUES
('PUE', 'Pago en una sola exhibición'),
('PPD', 'Pago en parcialidades o diferido')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

ALTER TABLE sat_metodos_pago ENABLE ROW LEVEL SECURITY;
CREATE POLICY sat_select_all ON sat_metodos_pago FOR SELECT USING (true);
