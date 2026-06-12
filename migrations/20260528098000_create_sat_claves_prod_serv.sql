-- Migración para crear el catálogo oficial del SAT para Claves de Productos y Servicios.
-- Incluye la definición de la tabla y precarga de claves esenciales y comunes para facturación.

-- 1. Crear la tabla de claves de productos y servicios del SAT
CREATE TABLE IF NOT EXISTS sat_claves_prod_serv (
    clave        VARCHAR(8) PRIMARY KEY,
    descripcion  TEXT NOT NULL,
    activo       BOOLEAN NOT NULL DEFAULT TRUE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 2. Crear índices B-Tree para búsquedas eficientes por descripción y clave
CREATE INDEX IF NOT EXISTS idx_sat_claves_desc ON sat_claves_prod_serv(descripcion);

-- 3. Precargar catálogo con claves comunes e indispensables del SAT (c_ClaveProdServ)
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
ON CONFLICT (clave) DO UPDATE SET 
    descripcion = EXCLUDED.descripcion;
