use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use rust_template::router;
use tower::ServiceExt;

#[tokio::test]
async fn hello_endpoint_returns_greeting() {
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
