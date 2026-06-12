use sqlx::PgPool;
use uuid::Uuid;

use crate::auth;
use crate::errors::AppError;
use crate::models::{CreateUsuarioDto, UpdateUsuarioDto, Usuario};

pub async fn listar(pool: &PgPool, empresa_id: Uuid) -> Result<Vec<Usuario>, AppError> {
    let rows = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT u.id, u.nombre, u.email, u.password_hash, u.activo, u.es_admin_plataforma, u.created_at, u.updated_at
        FROM usuarios u
        INNER JOIN perfiles p ON p.usuario_id = u.id
        WHERE p.empresa_id = $1 AND p.activo = true
        ORDER BY u.nombre ASC
        "#,
    )
    .bind(empresa_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn obtener(pool: &PgPool, id: Uuid) -> Result<Usuario, AppError> {
    let row = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT id, nombre, email, password_hash, activo, es_admin_plataforma, created_at, updated_at
        FROM usuarios
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn buscar_por_email(pool: &PgPool, email: &str) -> Result<Usuario, AppError> {
    let row = sqlx::query_as::<_, Usuario>(
        r#"
        SELECT id, nombre, email, password_hash, activo, es_admin_plataforma, created_at, updated_at
        FROM usuarios
        WHERE email = $1
        "#,
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn crear(pool: &PgPool, dto: CreateUsuarioDto) -> Result<Usuario, AppError> {
    auth::validar_contrasena_fuerte(&dto.password)?;

    let password_hash = auth::hashear_password(&dto.password)?;

    let row = sqlx::query_as::<_, Usuario>(
        r#"
        INSERT INTO usuarios (nombre, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, nombre, email, password_hash, activo, es_admin_plataforma, created_at, updated_at
        "#,
    )
    .bind(dto.nombre)
    .bind(dto.email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;

    Ok(row)
}


pub async fn actualizar(pool: &PgPool, id: Uuid, dto: UpdateUsuarioDto) -> Result<Usuario, AppError> {
    let row = sqlx::query_as::<_, Usuario>(
        r#"
        UPDATE usuarios SET
            nombre     = COALESCE($2, nombre),
            email      = COALESCE($3, email),
            activo     = COALESCE($4, activo),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, nombre, email, password_hash, activo, es_admin_plataforma, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(dto.nombre)
    .bind(dto.email)
    .bind(dto.activo)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn actualizar_password_hash(pool: &PgPool, id: Uuid, nuevo_hash: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE usuarios SET password_hash = $1, updated_at = NOW() WHERE id = $2")
        .bind(nuevo_hash)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn eliminar(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let resultado = sqlx::query("DELETE FROM usuarios WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if resultado.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Usuario con id {} no encontrado", id)));
    }

    Ok(())
}

pub async fn obtener_perfil_completo(pool: &PgPool, usuario_id: Uuid) -> Result<crate::models::ProfileResponse, AppError> {
    let usuario = obtener(pool, usuario_id).await?;

    let datos_empleado = sqlx::query_as::<_, crate::models::PerfilEmpleado>(
        r#"
        SELECT id, nombre_completo, telefono, puesto, curp, nss, fecha_nacimiento, fecha_ingreso, direccion, created_at, updated_at
        FROM perfiles_empleados
        WHERE id = $1
        "#,
    )
    .bind(usuario_id)
    .fetch_optional(pool)
    .await?;

    let roles_asignados = sqlx::query_as::<_, crate::models::RolAsignadoDetalle>(
        r#"
        SELECT 
            p.id as perfil_id,
            p.empresa_id,
            e.nombre as empresa_nombre,
            p.sucursal_id,
            s.nombre as sucursal_nombre,
            p.nivel
        FROM perfiles p
        INNER JOIN empresas e ON e.id = p.empresa_id
        LEFT JOIN sucursales s ON s.id = p.sucursal_id
        WHERE p.usuario_id = $1 AND p.activo = true
        ORDER BY e.nombre, s.nombre
        "#,
    )
    .bind(usuario_id)
    .fetch_all(pool)
    .await?;

    Ok(crate::models::ProfileResponse {
        usuario,
        datos_empleado,
        roles_asignados,
    })
}
