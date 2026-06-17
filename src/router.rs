use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::handlers::{admin_handler, auth_handler, empresa_handler, health_handler, perfil_handler, sat_handler, sse_handler, sucursal_handler, sync_handler, usuario_handler};
use crate::state::AppState;

/// Construye y devuelve el Router de Axum con todas las rutas CRUD registradas.
/// Responsabilidad única: definir el árbol de rutas (SRP).
/// Abierto a extensión: agregar nuevas rutas no requiere modificar handlers ni main (OCP).
pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler::health_check))
        // Auth
        .route("/auth/login", post(auth_handler::login))
        // Empresas CRUD
        .route(
            "/empresas",
            get(empresa_handler::listar_empresas).post(empresa_handler::crear_empresa),
        )
        .route(
            "/empresas/{id}",
            get(empresa_handler::obtener_empresa)
                .patch(empresa_handler::actualizar_empresa)
                .delete(empresa_handler::eliminar_empresa),
        )
        // Sucursales CRUD
        .route(
            "/sucursales",
            get(sucursal_handler::listar_sucursales).post(sucursal_handler::crear_sucursal),
        )
        .route(
            "/sucursales/{id}",
            get(sucursal_handler::obtener_sucursal)
                .patch(sucursal_handler::actualizar_sucursal)
                .delete(sucursal_handler::eliminar_sucursal),
        )
        // Usuarios CRUD
        .route(
            "/usuarios",
            get(usuario_handler::listar_usuarios).post(usuario_handler::crear_usuario),
        )
        .route(
            "/usuarios/{id}",
            get(usuario_handler::obtener_usuario)
                .patch(usuario_handler::actualizar_usuario)
                .delete(usuario_handler::eliminar_usuario),
        )
        // Perfiles (pivot usuario ↔ empresa/sucursal)
        // GET /perfiles?usuario_id=... o ?empresa_id=...
        .route(
            "/perfiles",
            get(perfil_handler::listar_perfiles).post(perfil_handler::crear_perfil),
        )
        .route(
            "/perfiles/{id}",
            get(perfil_handler::obtener_perfil)
                .patch(perfil_handler::actualizar_perfil)
                .delete(perfil_handler::eliminar_perfil),
        )
        // Perfil completo del usuario autenticado
        .route("/profile", get(usuario_handler::obtener_mi_perfil))
        // Catálogo oficial del SAT
        .route("/sat/claves", get(sat_handler::buscar_claves))
        .route("/sat/unidades", get(sat_handler::buscar_unidades))
        .route("/sat/formas-pago", get(sat_handler::buscar_formas_pago))
        .route("/sat/metodos-pago", get(sat_handler::buscar_metodos_pago))
        .route("/sat/usos-cfdi", get(sat_handler::buscar_usos_cfdi))
        .route("/sat/regimenes-fiscales", get(sat_handler::buscar_regimenes_fiscales))
        .route("/sat/tipos-comprobante", get(sat_handler::buscar_tipos_comprobante))
        .route("/sat/exportacion", get(sat_handler::buscar_exportacion))
        // Sincronización offline para dispositivos móviles
        .route("/sync", post(sync_handler::sync_batch))
        // Realtime: alertas de stock vía SSE
        .route("/realtime/stock", get(sse_handler::stream_stock_alerts))
        // Admin de plataforma: gestión global del SaaS
        .route(
            "/admin/duenos",
            get(admin_handler::listar_duenos),
        )
        .layer(middleware::from_fn(crate::middleware::log_request_response))
        .with_state(state)
}




