use axum::{
    extract::{Query, State},
    response::sse::{Event, KeepAlive, Sse},
};
use futures::stream;
use serde::Deserialize;
use tokio::sync::broadcast::error::RecvError;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct StockAlertsParams {
    sucursal_id: Option<Uuid>,
}

/// Suscripción SSE a alertas de stock bajos o agotados.
///
/// `GET /realtime/stock?sucursal_id=...`
pub async fn stream_stock_alerts(
    State(state): State<AppState>,
    Query(params): Query<StockAlertsParams>,
) -> Sse<impl stream::Stream<Item = Result<Event, std::convert::Infallible>>> {
    let rx = state.stock_tx.subscribe();
    let filter = params.sucursal_id;

    let stream = stream::unfold(rx, move |mut rx| async move {
        loop {
            match rx.recv().await {
                Ok(event) => {
                    // Filtrar por sucursal si se especificó
                    if let Some(suc) = filter {
                        if event.sucursal_id != suc {
                            continue;
                        }
                    }
                    let data = serde_json::to_string(&event).unwrap_or_default();
                    let sse_event = Event::default()
                        .event("stock_alert")
                        .data(data)
                        .id(event.timestamp.timestamp_millis().to_string());
                    return Some((Ok(sse_event), rx));
                }
                Err(RecvError::Lagged(n)) => {
                    let sse_event = Event::default()
                        .event("reconnect")
                        .data(format!("Se perdieron {} eventos", n));
                    return Some((Ok(sse_event), rx));
                }
                Err(RecvError::Closed) => {
                    return None;
                }
            }
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("keep-alive"),
    )
}
