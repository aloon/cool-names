use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use cool_names::NameGenerator;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    generator: Arc<NameGenerator>,
}

#[derive(Serialize)]
struct NameResponse {
    name: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn accepts_html(headers: &HeaderMap) -> bool {
    headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .map(|accept| accept.contains("text/html"))
        .unwrap_or(true) // Default to HTML
}

fn create_html_response(name: &str) -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Cool Names</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}

        .container {{
            background: rgba(255, 255, 255, 0.95);
            border-radius: 20px;
            padding: 60px 40px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            max-width: 800px;
            width: 100%;
            text-align: center;
        }}

        h1 {{
            color: #667eea;
            font-size: 3em;
            margin-bottom: 40px;
            text-transform: uppercase;
            letter-spacing: 3px;
            font-weight: 700;
        }}

        .name-box {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            border-radius: 15px;
            font-size: 2.5em;
            font-weight: bold;
            box-shadow: 0 10px 30px rgba(102, 126, 234, 0.4);
            letter-spacing: 2px;
            word-break: break-word;
        }}

        .refresh-btn {{
            margin-top: 30px;
            padding: 15px 40px;
            font-size: 1.1em;
            background: #667eea;
            color: white;
            border: none;
            border-radius: 50px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.3s ease;
            box-shadow: 0 5px 15px rgba(102, 126, 234, 0.3);
        }}

        .refresh-btn:hover {{
            background: #764ba2;
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(118, 75, 162, 0.4);
        }}

        @media (max-width: 600px) {{
            h1 {{
                font-size: 2em;
            }}

            .name-box {{
                font-size: 1.8em;
                padding: 30px;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Cool Names</h1>
        <div class="name-box">{}</div>
        <button class="refresh-btn" onclick="location.reload()">Generate Another</button>
    </div>
</body>
</html>"#,
        name
    ))
}

async fn generate_name(State(state): State<AppState>, headers: HeaderMap) -> Response {
    match state.generator.generate() {
        Ok(name) => {
            if accepts_html(&headers) {
                (StatusCode::OK, create_html_response(&name)).into_response()
            } else {
                (StatusCode::OK, Json(NameResponse { name })).into_response()
            }
        }
        Err(e) => {
            let error_msg = e.to_string();
            if accepts_html(&headers) {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html(format!(
                        "<html><body><h1>Error</h1><p>{}</p></body></html>",
                        error_msg
                    )),
                )
                    .into_response()
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse { error: error_msg }),
                )
                    .into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let generator = match NameGenerator::new() {
        Ok(gen) => Arc::new(gen),
        Err(e) => {
            eprintln!("Error initializing generator: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState { generator };
    let app = Router::new()
        .route("/", get(generate_name))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3002")
        .await
        .expect("Failed to bind to port 3002");

    println!("Server running on http://127.0.0.1:3002");
    println!("Try: curl http://127.0.0.1:3002/");

    axum::serve(listener, app).await.expect("Server error");
}
