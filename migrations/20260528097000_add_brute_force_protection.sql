-- Migración para añadir protección contra ataques de fuerza bruta en la tabla 'usuarios'
-- mediante el rastreo de intentos fallidos y el bloqueo temporal de cuentas.

-- 1. Añadir columnas de auditoría de intentos a la tabla 'usuarios'
ALTER TABLE usuarios ADD COLUMN IF NOT EXISTS intentos_fallidos INT NOT NULL DEFAULT 0;
ALTER TABLE usuarios ADD COLUMN IF NOT EXISTS bloqueado_hasta TIMESTAMPTZ;

-- 2. Asegurar que las cuentas bloqueadas se limpien automáticamente o se consideren en auditorías
CREATE INDEX IF NOT EXISTS idx_usuarios_bloqueo ON usuarios(bloqueado_hasta) WHERE bloqueado_hasta IS NOT NULL;
