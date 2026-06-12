-- Migración para crear el catálogo oficial del SAT de Unidades de Medida (c_ClaveUnidad) para CFDI 4.0.
-- Define la tabla de almacenamiento y precarga de unidades de medida estándar.

-- 1. Crear la tabla de unidades de medida del SAT
CREATE TABLE IF NOT EXISTS sat_unidades_medida (
    clave        VARCHAR(10) PRIMARY KEY, -- Soporta claves alfanuméricas del SAT
    nombre       VARCHAR(150) NOT NULL,
    descripcion  TEXT,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Crear índice para optimizar búsquedas por nombre de la unidad
CREATE INDEX IF NOT EXISTS idx_sat_unidades_nombre ON sat_unidades_medida(nombre);

-- 3. Precargar catálogo con unidades de medida indispensables para la facturación mexicana
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
