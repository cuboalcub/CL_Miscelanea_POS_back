-- Migración para crear el catálogo oficial del SAT de Tipos de Comprobante (c_TipoComprobante) para CFDI 4.0.
CREATE TABLE IF NOT EXISTS sat_tipos_comprobante (
    clave        VARCHAR(10) PRIMARY KEY,
    descripcion  VARCHAR(255) NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sat_tipos_comprobante_descripcion ON sat_tipos_comprobante(descripcion);

INSERT INTO sat_tipos_comprobante (clave, descripcion) VALUES
('I', 'Ingreso'),
('E', 'Egreso'),
('T', 'Traslado'),
('N', 'Nómina'),
('P', 'Pago')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

ALTER TABLE sat_tipos_comprobante ENABLE ROW LEVEL SECURITY;
CREATE POLICY sat_select_all ON sat_tipos_comprobante FOR SELECT USING (true);
