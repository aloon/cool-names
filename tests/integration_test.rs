use axum::body::Body;
use axum::http::{Request, StatusCode};
use cool_names::adapters::create_router;
use cool_names::application::NameGeneratorService;
use cool_names::domain::ports::NameGenerator;
use cool_names::infrastructure::FileWordRepository;
use std::path::Path;
use std::sync::Arc;
use tower::util::ServiceExt;

#[tokio::test]
async fn test_full_integration_with_real_files() {
    let adjectives_path = Path::new("adjectives.txt");
    let nouns_path = Path::new("nouns.txt");

    let repository = Arc::new(
        FileWordRepository::new(adjectives_path, nouns_path).expect("Failed to load word files"),
    );

    let name_generator = Arc::new(NameGeneratorService::new(repository.clone()));
    let app = create_router(name_generator);

    let response = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();

    assert!(body_str.contains("name"));
    assert!(body_str.contains("-"));
}

#[tokio::test]
async fn test_multiple_requests_return_different_names() {
    let adjectives_path = Path::new("adjectives.txt");
    let nouns_path = Path::new("nouns.txt");

    let repository = Arc::new(
        FileWordRepository::new(adjectives_path, nouns_path).expect("Failed to load word files"),
    );

    let name_generator = Arc::new(NameGeneratorService::new(repository));

    let mut names = std::collections::HashSet::new();

    for _ in 0..20 {
        let cool_name = name_generator.generate().unwrap();
        names.insert(cool_name.to_string());
    }

    assert!(
        names.len() > 1,
        "Should generate different names across multiple calls"
    );
}
