CREATE TABLE compras (
    id           UUID PRIMARY KEY,
    folio        VARCHAR(30) NOT NULL,
    proveedor    VARCHAR(255),
    sucursal_id  UUID NOT NULL REFERENCES sucursales(id),
    usuario_id   UUID NOT NULL REFERENCES usuarios(id),
    subtotal     DOUBLE PRECISION NOT NULL DEFAULT 0,
    iva          DOUBLE PRECISION NOT NULL DEFAULT 0,
    total        DOUBLE PRECISION NOT NULL DEFAULT 0,
    empresa_id   UUID NOT NULL,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_compras_empresa  ON compras(empresa_id);
CREATE INDEX idx_compras_sucursal ON compras(sucursal_id);
CREATE INDEX idx_compras_folio    ON compras(empresa_id, folio);

CREATE TABLE compras_detalle (
    id             UUID PRIMARY KEY,
    compra_id      UUID NOT NULL REFERENCES compras(id) ON DELETE CASCADE,
    producto_id    UUID NOT NULL REFERENCES productos(id),
    cantidad       DOUBLE PRECISION NOT NULL,
    precio_unitario DOUBLE PRECISION NOT NULL,
    importe        DOUBLE PRECISION NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_compras_detalle_compra ON compras_detalle(compra_id);
