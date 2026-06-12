# CL Miscelánea POS — Backend

Backend de Punto de Venta (POS) para misceláneas, construido en Rust. Arquitectura multitenencia (SaaS) con sincronización offline-first, catálogos oficiales del SAT y alertas de stock en tiempo real.

## Características principales

- **Multitenencia** — Cada empresa es un tenant independiente con su propio subdominio, sucursales, usuarios, productos e inventario.
- **Sincronización offline-first** — Endpoint `POST /sync` para que dispositivos móviles POS operen sin conexión y sincronicen en lotes (productos, clientes) con resolución de conflictos LWW (Last-Writer-Wins).
- **Alertas de stock en tiempo real** — SSE en `/realtime/stock` notifica cuando el inventario baja del mínimo o se agota.
- **Catálogos SAT** — Los 8 catálogos oficiales de CFDI 4.0 precargados en la base de datos (claves de producto/servicio, unidades, formas de pago, métodos de pago, usos CFDI, regímenes fiscales, tipos de comprobante, claves de exportación).
- **RBAC** — Control de acceso por niveles: SuperAdmin, Admin, Cajero, Consulta.
- **Administración de plataforma** — Gestión SaaS para listar dueños y empresas.

## Tecnologías

| Tecnología | Propósito |
|---|---|
| Rust (edition 2024) | Lenguaje principal |
| Axum 0.7 | Framework HTTP |
| SQLx 0.7 | Driver PostgreSQL asíncrono con migraciones |
| Tokio | Runtime asíncrono |
| Argon2 | Hashing de contraseñas |
| jsonwebtoken | JWT |
| PostgreSQL 16 | Base de datos |
| Docker Compose | Base de datos contenerizada |

## Requisitos

- Docker y Docker Compose
- Rust toolchain (edition 2024, estable reciente o nightly)

## Inicio rápido

```bash
# 1. Iniciar PostgreSQL
docker compose up -d

# 2. Sembrar catálogos SAT (una sola vez)
psql -d postgres://postgres:postgres@127.0.0.1:5432/cl_miscelanea_pos -f seeds/sat_catalogos.sql

# 3. Ejecutar el servidor
cargo run
```

El servidor arranca en `http://127.0.0.1:8080`. Las migraciones (26 archivos) se ejecutan automáticamente al iniciar.

## Variables de entorno

| Variable | Valor por defecto |
|---|---|
| `DATABASE_URL` | `postgres://postgres:postgres@127.0.0.1:5432/cl_miscelanea_pos` |
| `PORT` | `8080` |
| `HOST` | `127.0.0.1` |
| `JWT_SECRET` | `cl_miscelanea_pos_jwt_secret_super_seguro_2026` |
| `JWT_EXPIRATION_HOURS` | `24` |

## API

### Salud

| Método | Ruta | Auth |
|---|---|---|
| GET | `/health` | No |

### Autenticación

| Método | Ruta | Auth |
|---|---|---|
| POST | `/auth/login` | No |

### Empresas

| Método | Ruta | Auth |
|---|---|---|
| GET | `/empresas` | — |
| POST | `/empresas` | SuperAdmin |
| GET | `/empresas/{id}` | — |
| PATCH | `/empresas/{id}` | SuperAdmin |
| DELETE | `/empresas/{id}` | SuperAdmin |

### Sucursales

| Método | Ruta |
|---|---|
| GET | `/sucursales` |
| POST | `/sucursales` |
| GET | `/sucursales/{id}` |
| PATCH | `/sucursales/{id}` |
| DELETE | `/sucursales/{id}` |

### Usuarios

| Método | Ruta |
|---|---|
| GET | `/usuarios` |
| POST | `/usuarios` |
| GET | `/usuarios/{id}` |
| PATCH | `/usuarios/{id}` |
| DELETE | `/usuarios/{id}` |
| GET | `/profile` |

### Perfiles (RBAC)

| Método | Ruta |
|---|---|
| GET | `/perfiles?usuario_id=&empresa_id=` |
| POST | `/perfiles` |
| GET | `/perfiles/{id}` |
| PATCH | `/perfiles/{id}` |
| DELETE | `/perfiles/{id}` |

### Catálogos SAT

| Método | Ruta | Descripción |
|---|---|---|
| GET | `/sat/claves` | Claves de producto/servicio |
| GET | `/sat/unidades` | Unidades de medida |
| GET | `/sat/formas-pago` | Formas de pago |
| GET | `/sat/metodos-pago` | Métodos de pago |
| GET | `/sat/usos-cfdi` | Usos de CFDI |
| GET | `/sat/regimenes-fiscales` | Regímenes fiscales |
| GET | `/sat/tipos-comprobante` | Tipos de comprobante |
| GET | `/sat/exportacion` | Claves de exportación |

Todos aceptan `?q=` para búsqueda.

### Sincronización

| Método | Ruta | Descripción |
|---|---|---|
| POST | `/sync` | Sincronización batch offline (máx. 200 ops) |

### Tiempo real

| Método | Ruta | Descripción |
|---|---|---|
| GET | `/realtime/stock?sucursal_id=` | SSE de alertas de stock |

### Administración

| Método | Ruta | Auth |
|---|---|---|
| GET | `/admin/duenos` | Platform Admin |

## Estructura del proyecto

```
├── src/
│   ├── main.rs                # Punto de entrada
│   ├── router.rs              # Definición de rutas
│   ├── state.rs               # Estado compartido (AppState)
│   ├── auth.rs                # JWT y autenticación
│   ├── errors.rs              # Manejo centralizado de errores
│   ├── realtime.rs            # Lógica de alertas de stock
│   ├── handlers/              # Handlers HTTP
│   ├── models/                # Modelos de datos y DTOs
│   ├── repositories/          # Capa de acceso a datos
│   └── processors/            # Procesadores de negocio (sync)
├── migrations/                # Migraciones SQL (26 archivos)
├── seeds/                     # Semillas de catálogos SAT
├── bruno/                     # Colección de pruebas API (Bruno)
├── docker-compose.yml         # PostgreSQL contenerizado
└── Cargo.toml                 # Manifiesto de Rust
```

## Seguridad

- Contraseñas con Argon2 (resistente a ataques GPU)
- Política de contraseñas fuertes (8+ caracteres, mayúscula, minúscula, número, especial)
- JWT con expiración configurable
- RLS (Row-Level Security) en catálogos SAT
- RBAC con 4 niveles de acceso
- Protección contra fuerza bruta
- Migración desde hashes legado (`HASH_PLACEHOLDER:`)

## Licencia

Uso interno.
