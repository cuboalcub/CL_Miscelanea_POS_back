use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncAuditLog {
    pub id: Uuid,
    pub dispositivo_id: Uuid,
    pub sucursal_id: Uuid,
    pub usuario_id: Option<Uuid>,
    pub empresa_id: Uuid,
    pub total_operaciones: i32,
    pub operaciones_ok: i32,
    pub operaciones_error: i32,
    pub primer_error: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SyncOperation {
    pub id: Uuid,
    pub dispositivo_id: Uuid,
    pub sucursal_id: Uuid,
    pub usuario_id: Option<Uuid>,
    pub empresa_id: Uuid,
    pub cliente_id: String,
    pub tipo_operacion: String,
    pub entidad: String,
    pub datos: serde_json::Value,
    pub estado: String,
    pub resultado: Option<serde_json::Value>,
    pub ocurrido_en: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub procesado_en: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SyncBatchRequest {
    pub operaciones: Vec<SyncOperationRequest>,
    pub dispositivo_id: Uuid,
    pub sucursal_id: Uuid,
    pub ultimo_sync: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct SyncOperationRequest {
    pub cliente_id: String,
    pub tipo: String,
    pub entidad: String,
    pub datos: serde_json::Value,
    #[serde(default = "Utc::now")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SyncBatchResponse {
    pub resultados: Vec<SyncOperationResult>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SyncOperationResult {
    pub cliente_id: String,
    pub estado: String,
    pub sync_operation_id: Option<Uuid>,
    pub error: Option<SyncErrorDetail>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct SyncErrorDetail {
    pub codigo: String,
    pub mensaje: String,
    pub sugerencia: Option<String>,
}
