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
    <title>COOL-NAMES-GEN v0.1.0 | TERMINAL</title>
    <style>
        @import url('https://fonts.googleapis.com/css2?family=VT323&display=swap');

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'VT323', 'Courier New', monospace;
            background: #000000;
            color: #33ff33;
            min-height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            padding: 20px;
            font-size: 20px;
            position: relative;
            overflow: hidden;
        }}

        body::before {{
            content: "";
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background:
                repeating-linear-gradient(
                    0deg,
                    rgba(0, 0, 0, 0.15),
                    rgba(0, 0, 0, 0.15) 1px,
                    transparent 1px,
                    transparent 2px
                );
            pointer-events: none;
            z-index: 999;
        }}

        body::after {{
            content: "";
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: radial-gradient(ellipse at center, transparent 0%, rgba(0,0,0,0.3) 100%);
            pointer-events: none;
            z-index: 998;
        }}

        .terminal {{
            background: #000000;
            border: 3px solid #33ff33;
            padding: 30px;
            max-width: 900px;
            width: 100%;
            box-shadow: 0 0 40px rgba(51, 255, 51, 0.4),
                        inset 0 0 100px rgba(51, 255, 51, 0.03);
            position: relative;
            z-index: 1;
        }}

        .ascii-header {{
            color: #00ff00;
            font-size: 0.8em;
            line-height: 1.2;
            margin-bottom: 20px;
            text-align: center;
            text-shadow: 0 0 5px rgba(0, 255, 0, 0.7);
        }}

        .system-info {{
            color: #33ff33;
            font-size: 0.9em;
            margin-bottom: 20px;
            opacity: 0.8;
        }}

        .info-line {{
            margin: 5px 0;
        }}

        .separator {{
            color: #33ff33;
            margin: 20px 0;
            opacity: 0.5;
        }}

        .prompt {{
            color: #33ff33;
            margin: 10px 0;
        }}

        .prompt::before {{
            content: "root@localhost:~# ";
            color: #00ff00;
            font-weight: bold;
        }}

        .output {{
            color: #33ff33;
            margin: 10px 0 10px 20px;
            opacity: 0.9;
        }}

        .name-box {{
            background: #001100;
            color: #00ff00;
            padding: 25px;
            border: 2px solid #33ff33;
            font-size: 2.2em;
            font-weight: bold;
            text-align: center;
            letter-spacing: 5px;
            word-break: break-word;
            text-shadow: 0 0 15px rgba(0, 255, 0, 0.8);
            margin: 20px 0;
            position: relative;
        }}

        .name-box::before {{
            content: ">> ";
            color: #00ff00;
        }}

        .name-box::after {{
            content: " <<";
            color: #00ff00;
        }}

        .command {{
            margin-top: 20px;
            padding: 10px 25px;
            font-size: 1.1em;
            background: transparent;
            color: #33ff33;
            border: 2px solid #33ff33;
            cursor: pointer;
            font-family: 'VT323', monospace;
            font-weight: 600;
            transition: all 0.1s ease;
            letter-spacing: 2px;
            display: block;
            width: 100%;
        }}

        .command:hover {{
            background: #33ff33;
            color: #000000;
            box-shadow: 0 0 25px rgba(51, 255, 51, 0.6);
            text-shadow: none;
        }}

        .cursor {{
            display: inline-block;
            width: 12px;
            height: 22px;
            background: #33ff33;
            animation: blink 0.8s infinite;
            margin-left: 5px;
            box-shadow: 0 0 5px rgba(51, 255, 51, 0.8);
        }}

        @keyframes blink {{
            0%, 49% {{ opacity: 1; }}
            50%, 100% {{ opacity: 0; }}
        }}

        .footer {{
            margin-top: 20px;
            color: #33ff33;
            font-size: 0.85em;
            opacity: 0.6;
            text-align: center;
        }}

        @media (max-width: 600px) {{
            .terminal {{
                padding: 20px;
            }}

            .ascii-header {{
                font-size: 0.6em;
            }}

            .name-box {{
                font-size: 1.5em;
                padding: 15px;
                letter-spacing: 2px;
            }}
        }}
    </style>
</head>
<body>
    <div class="terminal">
        <pre class="ascii-header">
 ██████╗ ██████╗  ██████╗ ██╗         ███╗   ██╗ █████╗ ███╗   ███╗███████╗███████╗
██╔════╝██╔═══██╗██╔═══██╗██║         ████╗  ██║██╔══██╗████╗ ████║██╔════╝██╔════╝
██║     ██║   ██║██║   ██║██║         ██╔██╗ ██║███████║██╔████╔██║█████╗  ███████╗
██║     ██║   ██║██║   ██║██║         ██║╚██╗██║██╔══██║██║╚██╔╝██║██╔══╝  ╚════██║
╚██████╗╚██████╔╝╚██████╔╝███████╗    ██║ ╚████║██║  ██║██║ ╚═╝ ██║███████╗███████║
 ╚═════╝ ╚═════╝  ╚═════╝ ╚══════╝    ╚═╝  ╚═══╝╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝╚══════╝
        </pre>

        <div class="system-info">
            <div class="info-line">[SYSTEM] Name Generator v0.1.0 | Build: RELEASE</div>
            <div class="info-line">[STATUS] Connection established | Protocol: HTTP/1.1</div>
            <div class="info-line">[TIME] <script>document.write(new Date().toISOString())</script></div>
        </div>

        <div class="separator">════════════════════════════════════════════════════════════════════</div>

        <div class="prompt">./name-generator --mode random --output stdout</div>
        <div class="output">[INFO] Initializing random name generator...</div>
        <div class="output">[INFO] Loading adjective database... OK</div>
        <div class="output">[INFO] Loading noun database... OK</div>
        <div class="output">[SUCCESS] Name generated successfully!</div>

        <div class="name-box">{}</div>

        <div class="output">[STATS] Generation time: 0.{}ms | Entropy: HIGH</div>

        <div class="prompt">
            <span class="cursor"></span>
        </div>

        <button class="command" onclick="location.reload()">[EXECUTE] ./generate-new-name.sh</button>

        <div class="footer">
            ┌────────────────────────────────────────────────┐<br>
            │ Press ENTER to generate another cool name      │<br>
            │ [ESC] Exit | [F1] Help | [F5] Refresh         │<br>
            └────────────────────────────────────────────────┘
        </div>
    </div>
    <script>
        document.addEventListener('keydown', function(e) {{
            if (e.key === 'Enter' || e.key === 'F5') {{
                e.preventDefault();
                location.reload();
            }}
        }});
    </script>
</body>
</html>"#,
        name,
        (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() % 1000)
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
