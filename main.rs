use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Clone)]
struct CFramePayload {
    cframe: Vec<f32>,
}

struct AppState {
    current_cframe: RwLock<Vec<f32>>,
}

#[tokio::main]
async fn main() {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap();

    let shared_state = Arc::new(AppState {
        current_cframe: RwLock::new(vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]),
    });

    let app = Router::new()
        .route("/update-cframe", post(update_cframe))
        .route("/get-cframe", get(get_cframe))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn update_cframe(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CFramePayload>,
) -> StatusCode {
    let mut current = state.current_cframe.write().unwrap();
    *current = payload.cframe;
    StatusCode::OK
}

async fn get_cframe(State(state): State<Arc<AppState>>) -> Json<CFramePayload> {
    let current = state.current_cframe.read().unwrap();
    Json(CFramePayload {
        cframe: current.clone(),
    })
}
