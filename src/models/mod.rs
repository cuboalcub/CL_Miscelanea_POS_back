pub mod categoria;
pub mod cliente;
pub mod compra;
pub mod empleado;
pub mod empresa;
pub mod inventario;
pub mod movimiento_stock;
pub mod perfil;
pub mod producto;
pub mod profile;
pub mod sat;
pub mod sucursal;
pub mod sync;
pub mod usuario;
pub mod venta;

#[allow(unused_imports)]
pub use empleado::{CreatePerfilEmpleadoDto, PerfilEmpleado, UpdatePerfilEmpleadoDto};
pub use empresa::{CreateEmpresaDto, DuenoInfo, Empresa, UpdateEmpresaDto};
#[allow(unused_imports)]
pub use perfil::{CreatePerfilDto, NivelAcceso, Perfil, UpdatePerfilDto};
pub use profile::{ProfileResponse, RolAsignadoDetalle};
pub use categoria::Categoria;
pub use cliente::Cliente;
pub use inventario::{CreateInventarioDto, Inventario, UpdateInventarioDto};
#[allow(unused_imports)]
pub use movimiento_stock::{CreateMovimientoStockDto, MovimientoStock, TipoMovimiento};
pub use producto::{CreateProductoDto, Producto, UpdateProductoDto};
pub use sat::{
    SatClaveProdServ, SatExportacion, SatFormaPago, SatMetodoPago, SatRegimenFiscal,
    SatTipoComprobante, SatUnidadMedida, SatUsoCfdi,
};
pub use sucursal::{CreateSucursalDto, Sucursal, UpdateSucursalDto};
pub use usuario::{CreateUsuarioDto, UpdateUsuarioDto, Usuario};
pub use venta::{
    CreateVentaRequest, Venta, VentaDetalle, VentaResponse,
};
pub use compra::{
    Compra, CompraDetalle, CompraResponse, CreateCompraRequest,
};

pub fn deserialize_present_option<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}



