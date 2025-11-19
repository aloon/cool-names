use cool_names::adapters::create_router;
use cool_names::application::NameGeneratorService;
use cool_names::domain::ports::WordRepository;
use cool_names::infrastructure::FileWordRepository;
use std::path::Path;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let adjectives_path = Path::new("adjectives.txt");
    let nouns_path = Path::new("nouns.txt");

    let repository = match FileWordRepository::new(adjectives_path, nouns_path) {
        Ok(repo) => Arc::new(repo),
        Err(e) => {
            eprintln!("Error loading word files: {}", e);
            std::process::exit(1);
        }
    };

    println!("Loaded {} adjectives and {} nouns",
        repository.adjectives_count(),
        repository.nouns_count()
    );

    let name_generator = Arc::new(NameGeneratorService::new(repository));
    let app = create_router(name_generator);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002")
        .await
        .expect("Failed to bind to port 3002");

    println!("Server running on http://127.0.0.1:3002");
    println!("Try: curl http://127.0.0.1:3002/");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
