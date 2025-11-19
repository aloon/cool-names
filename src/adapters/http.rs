use crate::domain::ports::NameGenerator;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    name_generator: Arc<dyn NameGenerator>,
}

impl AppState {
    pub fn new(name_generator: Arc<dyn NameGenerator>) -> Self {
        Self { name_generator }
    }
}

#[derive(Serialize)]
struct CoolNameResponse {
    name: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn generate_name_handler(State(state): State<AppState>) -> Response {
    match state.name_generator.generate() {
        Ok(cool_name) => (
            StatusCode::OK,
            Json(CoolNameResponse {
                name: cool_name.to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub fn create_router(name_generator: Arc<dyn NameGenerator>) -> Router {
    let state = AppState::new(name_generator);

    Router::new()
        .route("/", get(generate_name_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Adjective, CoolName, Noun};
    use crate::domain::ports::DomainError;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    struct MockNameGenerator {
        result: Result<CoolName, DomainError>,
    }

    impl NameGenerator for MockNameGenerator {
        fn generate(&self) -> Result<CoolName, DomainError> {
            self.result.clone()
        }
    }

    #[tokio::test]
    async fn test_get_root_returns_cool_name() {
        let mock_generator = Arc::new(MockNameGenerator {
            result: Ok(CoolName::new(
                Adjective::new("brave".to_string()),
                Noun::new("warrior".to_string()),
            )),
        });

        let app = create_router(mock_generator);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert!(body_str.contains("brave-warrior"));
    }

    #[tokio::test]
    async fn test_get_root_returns_error_on_failure() {
        let mock_generator = Arc::new(MockNameGenerator {
            result: Err(DomainError::NoAdjectivesAvailable),
        });

        let app = create_router(mock_generator);

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        assert!(body_str.contains("error"));
    }
}
