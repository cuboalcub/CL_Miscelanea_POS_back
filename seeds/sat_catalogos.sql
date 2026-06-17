-- ============================================================================
-- SEED COMPLETO: Catálogos Oficiales del SAT para CFDI 4.0
-- ============================================================================
-- Uso:   psql -d $DATABASE_URL -f seeds/sat_catalogos.sql
--        cargo run --bin seeder
--
-- Idempotente: seguro de ejecutar múltiples veces (usa ON CONFLICT DO UPDATE).
-- Pública todos los catálogos SAT necesarios para un sistema POS/facturación.
-- ============================================================================

-- ============================================================================
-- 1. c_FormaPago — Formas de Pago (tabla creada en migración 20260528100000)
-- ============================================================================
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
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

-- ============================================================================
-- 2. c_MetodoPago — Métodos de Pago (tabla creada en migración 20260528102000)
-- ============================================================================
INSERT INTO sat_metodos_pago (clave, descripcion) VALUES
('PUE', 'Pago en una sola exhibición'),
('PPD', 'Pago en parcialidades o diferido')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

-- ============================================================================
-- 3. c_UsoCFDI — Usos del CFDI (tabla creada en migración 20260528103000)
-- ============================================================================
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

-- ============================================================================
-- 4. c_RegimenFiscal — Regímenes Fiscales del Contribuyente (tabla creada en migración 20260528104000)
-- ============================================================================
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

-- ============================================================================
-- 5. c_TipoComprobante — Tipos de Comprobante Fiscal (tabla creada en migración 20260528105000)
-- ============================================================================
INSERT INTO sat_tipos_comprobante (clave, descripcion) VALUES
('I', 'Ingreso'),
('E', 'Egreso'),
('T', 'Traslado'),
('N', 'Nómina'),
('P', 'Pago')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

-- ============================================================================
-- 6. c_Exportacion — Claves de Exportación (tabla creada en migración 20260528106000)
-- ============================================================================
INSERT INTO sat_exportacion (clave, descripcion) VALUES
('01', 'No aplica'),
('02', 'Definitiva con clave A1'),
('03', 'Temporal'),
('04', 'Definitiva con clave distinta a A1 o A2')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

-- ============================================================================
-- 7. c_ClaveProdServ — Claves de Productos y Servicios (refresco, tabla creada en migración 20260528098000)
-- ============================================================================
INSERT INTO sat_claves_prod_serv (clave, descripcion) VALUES
('01010101', 'No existe en el catálogo'),
('50111500', 'Carne de res (Fresca o refrigerada)'),
('50131700', 'Leche y productos lácteos (Queso, crema, yogur)'),
('50161800', 'Confitería, dulces y chocolates'),
('50181900', 'Pan, galletas, pasteles y repostería'),
('50201700', 'Café, té y sucedáneos'),
('50202300', 'Bebidas no alcohólicas, refrescos y aguas minerales'),
('50202301', 'Agua purificada y de manantial'),
('43231500', 'Software de aplicación asistido por computadora (Sistemas de punto de venta / POS)'),
('80141628', 'Servicios de punto de venta'),
('84111506', 'Servicios de facturación'),
('50192100', 'Botanas, papas fritas y frutos secos procesados')
ON CONFLICT (clave) DO UPDATE SET descripcion = EXCLUDED.descripcion;

-- ============================================================================
-- 8. c_ClaveUnidad — Unidades de Medida (refresco, tabla creada en migración 20260528099000)
-- ============================================================================
INSERT INTO sat_unidades_medida (clave, nombre, descripcion) VALUES
('H87', 'Pieza', 'Unidad de conteo que define que el artículo se vende como una pieza única.'),
('EA', 'Elemento', 'Each - Cada uno. Comúnmente utilizado en retail e inventarios.'),
('KGM', 'Kilogramo', 'Unidad de masa del Sistema Internacional.'),
('LTR', 'Litro', 'Unidad de volumen para líquidos.'),
('E48', 'Unidad de servicio', 'Definición para conceptos que representan la prestación de un servicio.'),
('ACT', 'Actividad', 'Acción o trabajo realizado (servicios u honorarios).'),
('MTR', 'Metro', 'Unidad de longitud.'),
('MTQ', 'Metro cúbico', 'Unidad de volumen para sólidos y gases.'),
('XKI', 'Kit', 'Conjunto de elementos agrupados comercialmente.'),
('GRM', 'Gramo', 'Submúltiplo de masa.'),
('MLT', 'Mililitro', 'Submúltiplo de volumen.'),
('TNE', 'Tonelada métrica', 'Unidad de masa equivalente a 1000 kg.')
ON CONFLICT (clave) DO UPDATE SET
    nombre = EXCLUDED.nombre,
    descripcion = EXCLUDED.descripcion;
