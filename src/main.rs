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
    <title>Cool Names - Terminal</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;700&display=swap');

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'IBM Plex Mono', 'Courier New', monospace;
            background: #0a0a0a;
            color: #33ff33;
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
        }}

        .terminal {{
            background: #000000;
            border: 2px solid #33ff33;
            border-radius: 8px;
            padding: 40px;
            max-width: 800px;
            width: 100%;
            box-shadow: 0 0 30px rgba(51, 255, 51, 0.3),
                        inset 0 0 50px rgba(51, 255, 51, 0.05);
        }}

        .terminal-header {{
            color: #33ff33;
            font-size: 1em;
            margin-bottom: 30px;
            opacity: 0.7;
            letter-spacing: 1px;
        }}

        .prompt {{
            color: #33ff33;
            margin-bottom: 20px;
        }}

        .prompt::before {{
            content: "$ ";
            color: #00ff00;
            font-weight: bold;
        }}

        .name-box {{
            background: #0d1b0d;
            color: #00ff00;
            padding: 30px;
            border: 1px solid #33ff33;
            font-size: 2.5em;
            font-weight: bold;
            text-align: center;
            letter-spacing: 3px;
            word-break: break-word;
            text-shadow: 0 0 10px rgba(51, 255, 51, 0.5);
            margin: 20px 0;
        }}

        .command {{
            margin-top: 30px;
            padding: 12px 30px;
            font-size: 1em;
            background: transparent;
            color: #33ff33;
            border: 2px solid #33ff33;
            cursor: pointer;
            font-family: 'IBM Plex Mono', monospace;
            font-weight: 600;
            transition: all 0.2s ease;
            text-transform: uppercase;
            letter-spacing: 2px;
        }}

        .command:hover {{
            background: #33ff33;
            color: #000000;
            box-shadow: 0 0 20px rgba(51, 255, 51, 0.5);
        }}

        .cursor {{
            display: inline-block;
            width: 10px;
            height: 20px;
            background: #33ff33;
            animation: blink 1s infinite;
            margin-left: 5px;
        }}

        @keyframes blink {{
            0%, 49% {{ opacity: 1; }}
            50%, 100% {{ opacity: 0; }}
        }}

        @media (max-width: 600px) {{
            .terminal {{
                padding: 20px;
            }}

            .name-box {{
                font-size: 1.8em;
                padding: 20px;
            }}

            .command {{
                width: 100%;
            }}
        }}
    </style>
</head>
<body>
    <div class="terminal">
        <div class="terminal-header">[cool-names v0.1.0]</div>
        <div class="prompt">generate_name --random</div>
        <div class="name-box">{}</div>
        <div class="prompt">
            <span class="cursor"></span>
        </div>
        <button class="command" onclick="location.reload()">[ Generate New ]</button>
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
        .route("/api/name", get(generate_name))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002")
        .await
        .expect("Failed to bind to port 3002");

    println!("Server running on http://127.0.0.1:3002");
    println!("Try: curl http://127.0.0.1:3002/");

    axum::serve(listener, app).await.expect("Server error");
}
