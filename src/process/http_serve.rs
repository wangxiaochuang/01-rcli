use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Serving {:?} on {}", path, addr);
    let state = HttpServeState { path: path.clone() };
    let router = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", ServeDir::new(path))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}", p);
    if !p.exists() {
        (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html")],
            format!("File not found: {:?}", p.display()),
        )
    } else if p.is_file() {
        handle_file(p).await
    } else {
        handle_dir(p).await
    }
}

async fn handle_file(
    path: impl AsRef<std::path::Path>,
) -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    match tokio::fs::read_to_string(path).await {
        Ok(content) => {
            info!("Read {} bytes", content.len());
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/plain")],
                content,
            )
        }
        Err(e) => {
            warn!("Error reading file: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                [(header::CONTENT_TYPE, "text/plain")],
                e.to_string(),
            )
        }
    }
}

async fn handle_dir(
    path: impl AsRef<std::path::Path>,
) -> (StatusCode, [(header::HeaderName, &'static str); 1], String) {
    let mut readdir = tokio::fs::read_dir(path.as_ref()).await.unwrap();
    let mut content = String::new();
    while let Some(entry) = readdir.next_entry().await.unwrap() {
        let osname = entry.file_name();
        let fname = osname.into_string().unwrap();
        let ftype = entry.file_type().await.unwrap();

        if ftype.is_dir() {
            content.push_str(format!("<li><a href=\"{}/\">{}/</a></li>", fname, fname).as_ref());
        } else {
            content.push_str(format!("<li><a href=\"{}\">{}</a></li>", fname, fname).as_ref());
        }
    }
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html")],
        format!("<html><body><ul>{}</ul></body></html>", content),
    )
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::file_handler;
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
    };

    #[tokio::test]
    async fn test_file_handler() {
        let state = super::HttpServeState {
            path: std::path::PathBuf::from("."),
        };
        let res = file_handler(State(Arc::new(state)), Path("Cargo.toml".to_string())).await;
        let resp = res.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
