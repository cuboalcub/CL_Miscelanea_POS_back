CREATE TABLE ventas (
    id           UUID PRIMARY KEY,
    folio        VARCHAR(30) NOT NULL,
    cliente_id   UUID REFERENCES clientes(id),
    sucursal_id  UUID NOT NULL REFERENCES sucursales(id),
    usuario_id   UUID NOT NULL REFERENCES usuarios(id),
    subtotal     DOUBLE PRECISION NOT NULL DEFAULT 0,
    iva          DOUBLE PRECISION NOT NULL DEFAULT 0,
    total        DOUBLE PRECISION NOT NULL DEFAULT 0,
    forma_pago   VARCHAR(10),
    metodo_pago  VARCHAR(10),
    uso_cfdi     VARCHAR(10),
    estado       VARCHAR(20) NOT NULL DEFAULT 'completada',
    empresa_id   UUID NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_ventas_empresa  ON ventas(empresa_id);
CREATE INDEX idx_ventas_sucursal ON ventas(sucursal_id);
CREATE INDEX idx_ventas_folio    ON ventas(empresa_id, folio);

CREATE TABLE ventas_detalle (
    id             UUID PRIMARY KEY,
    venta_id       UUID NOT NULL REFERENCES ventas(id) ON DELETE CASCADE,
    producto_id    UUID NOT NULL REFERENCES productos(id),
    cantidad       DOUBLE PRECISION NOT NULL,
    precio_unitario DOUBLE PRECISION NOT NULL,
    importe        DOUBLE PRECISION NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_ventas_detalle_venta ON ventas_detalle(venta_id);
