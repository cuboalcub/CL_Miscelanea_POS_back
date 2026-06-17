-- Migración para crear el catálogo oficial del SAT de Claves de Exportación (c_Exportacion) para CFDI 4.0.
CREATE TABLE IF NOT EXISTS sat_exportacion (
    clave        VARCHAR(10) PRIMARY KEY,
    descripcion  VARCHAR(255) NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sat_exportacion_descripcion ON sat_exportacion(descripcion);

INSERT INTO sat_exportacion (clave, descripcion) VALUES
('01', 'No aplica'),
('02', 'Definitiva con clave A1'),
('03', 'Temporal'),
('04', 'Definitiva con clave distinta a A1 o A2')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

ALTER TABLE sat_exportacion ENABLE ROW LEVEL SECURITY;
CREATE POLICY sat_select_all ON sat_exportacion FOR SELECT USING (true);
