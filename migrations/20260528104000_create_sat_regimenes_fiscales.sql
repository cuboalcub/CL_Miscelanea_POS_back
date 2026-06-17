-- Migración para crear el catálogo oficial del SAT de Regímenes Fiscales (c_RegimenFiscal) para CFDI 4.0.
CREATE TABLE IF NOT EXISTS sat_regimenes_fiscales (
    clave        VARCHAR(10) PRIMARY KEY,
    descripcion  VARCHAR(255) NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sat_regimenes_fiscales_descripcion ON sat_regimenes_fiscales(descripcion);

INSERT INTO sat_regimenes_fiscales (clave, descripcion) VALUES
('601', 'General de Ley Personas Morales'),
('603', 'Personas Morales con Fines no Lucrativos'),
('605', 'Sueldos y Salarios e Ingresos Asimilados a Salarios'),
('606', 'Arrendamiento'),
('607', 'Régimen de Enajenación o Adquisición de Bienes'),
('608', 'Demás ingresos'),
('609', 'Consolidación'),
('610', 'Residentes en el Extranjero sin Establecimiento Permanente en México'),
('611', 'Ingresos por Dividendos (socios y accionistas)'),
('612', 'Personas Físicas con Actividades Empresariales y Profesionales'),
('614', 'Ingresos por intereses'),
('615', 'Régimen de los ingresos por obtención de premios'),
('616', 'Sin obligaciones fiscales'),
('620', 'Sociedades Cooperativas de Producción que optan por diferir sus ingresos'),
('621', 'Incorporación Fiscal'),
('622', 'Actividades Agrícolas, Ganaderas, Silvícolas y Pesqueras'),
('623', 'Opcional para Grupos de Sociedades'),
('624', 'Coordinados'),
('625', 'Régimen de las Actividades Empresariales con ingresos a través de Plataformas Tecnológicas'),
('626', 'Régimen Simplificado de Confianza')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

ALTER TABLE sat_regimenes_fiscales ENABLE ROW LEVEL SECURITY;
CREATE POLICY sat_select_all ON sat_regimenes_fiscales FOR SELECT USING (true);
