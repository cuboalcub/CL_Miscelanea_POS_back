-- Migración para crear el catálogo oficial del SAT de Usos del CFDI (c_UsoCFDI) para CFDI 4.0.
CREATE TABLE IF NOT EXISTS sat_usos_cfdi (
    clave        VARCHAR(10) PRIMARY KEY,
    descripcion  VARCHAR(255) NOT NULL,
    regimen_fiscal_aplica VARCHAR(500) NOT NULL DEFAULT 'NA',
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_sat_usos_cfdi_descripcion ON sat_usos_cfdi(descripcion);

INSERT INTO sat_usos_cfdi (clave, descripcion, regimen_fiscal_aplica) VALUES
('G01', 'Adquisición de mercancías', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('G02', 'Devoluciones, descuentos o bonificaciones', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('G03', 'Gastos en general', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I01', 'Construcciones', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I02', 'Mobiliario y equipo de oficina por inversiones', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I03', 'Equipo de transporte', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I04', 'Equipo de cómputo y accesorios', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I05', 'Dados, troqueles, moldes, matrices y herramental', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I06', 'Comunicaciones telefónicas', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I07', 'Comunicaciones satelitales', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('I08', 'Otra maquinaria y equipo', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D01', 'Honorarios médicos, dentales y gastos hospitalarios', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D02', 'Gastos médicos por incapacidad o discapacidad', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D03', 'Gastos funerarios', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D04', 'Donativos', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D05', 'Intereses reales efectivamente pagados por créditos hipotecarios', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D06', 'Aportaciones voluntarias al SAR', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D07', 'Primas por seguros de gastos médicos', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D08', 'Gastos de transportación escolar obligatoria', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D09', 'Depósitos en cuentas para el ahorro, primas que tengan como base planes de pensiones', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('D10', 'Pagos por servicios educativos (colegiaturas)', '605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('S01', 'Sin efectos fiscales', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('CP01', 'Pagos', '601,603,605,606,607,608,610,611,612,614,615,616,620,621,622,623,624,625,626'),
('CN01', 'Nómina', '605')
ON CONFLICT (clave) DO UPDATE SET
    descripcion = EXCLUDED.descripcion,
    regimen_fiscal_aplica = EXCLUDED.regimen_fiscal_aplica;

ALTER TABLE sat_usos_cfdi ENABLE ROW LEVEL SECURITY;
CREATE POLICY sat_select_all ON sat_usos_cfdi FOR SELECT USING (true);
