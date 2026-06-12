-- Otorga permisos al rol app_user sobre las tablas que se crearon
-- después de la migración original de grants.

GRANT INSERT, UPDATE, DELETE ON
    productos,
    clientes,
    categorias,
    sync_operations,
    sync_audit_log,
    inventario,
    movimientos_stock
TO app_user;
