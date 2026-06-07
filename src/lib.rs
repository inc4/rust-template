use axum::{Router, extract::Query, http::StatusCode, response::IntoResponse, routing::get};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new().route("/hello", get(handle_hello))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    reply: Option<String>,
}

#[tracing::instrument]
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    match params.reply.as_deref() {
        Some("not-found") => (StatusCode::NOT_FOUND, "not found").into_response(),
        Some("err") => (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response(),
        _ => (StatusCode::OK, "Hello".to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn hello_returns_greeting() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello");
    }

    #[tokio::test]
    async fn hello_returns_not_found_when_requested() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/hello?reply=not-found")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn hello_returns_error_when_requested() {
        let response = router()
            .oneshot(
                Request::builder()
                    .uri("/hello?reply=err")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
