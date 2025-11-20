# Cool Names

A simple Rust web service that generates random cool names by combining adjectives and nouns.

## Features

- **Random Name Generation**: Combines random adjectives with random nouns (52,719+ possible combinations)
- **Content Negotiation**: Returns HTML or JSON based on the `Accept` header
- **Beautiful UI**: Modern, responsive web interface with gradient design
- **Simple Architecture**: Clean, straightforward code structure
- **Comprehensive Testing**: Unit and integration tests

## Quick Start

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo

### Installation

```bash
git clone https://github.com/aloon/cool-names
cd cool-names
```

### Running the Application

#### With Cargo

```bash
cargo run
```

The server will start on `http://127.0.0.1:3002`

#### With Docker

Build the image:
```bash
docker build -t cool-names .
```

Run the container:
```bash
docker run -p 3002:3002 cool-names
```

Access the application at `http://localhost:3002`

### Testing

```bash
cargo test
```

## Usage

### Browser Access

Open your browser and navigate to:
```
http://127.0.0.1:3002/
```

You'll see a beautiful UI displaying a randomly generated name like "cosmic-dragon" or "blazing-phoenix".

### API Access (JSON)

Request JSON format using curl:

```bash
curl -H "Accept: application/json" http://127.0.0.1:3002/
```

Response:
```json
{
  "name": "mighty-warrior"
}
```

### Default (HTML)

Without specifying the Accept header (or with `Accept: text/html`), you'll receive HTML:

```bash
curl http://127.0.0.1:3002/
```

## Architecture

This project uses a simple, straightforward architecture:

```
src/
├── lib.rs    # Core logic: NameGenerator, word loading
└── main.rs   # Web server: Axum routes and handlers
```

### Components

1. **NameGenerator** (`lib.rs`): Core business logic
   - Loads adjectives and nouns from text files
   - Generates random name combinations
   - Handles errors gracefully

2. **Web Server** (`main.rs`): HTTP interface
   - Axum-based REST API
   - Content negotiation (HTML/JSON)
   - Beautiful HTML UI with CSS

## Configuration

### Word Lists

Words are loaded from:
- `adjectives.txt` - 221+ adjectives
- `nouns.txt` - 239+ nouns

These files are loaded into memory on startup for fast random access.

### Port Configuration

Change the port in `src/main.rs`:

```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3002")
```

## Dependencies

- **axum** (0.8): Modern web framework
- **tokio** (1.x): Async runtime
- **rand** (0.9): Random number generation
- **thiserror** (2.0): Error handling
- **serde** (1.0): Serialization/deserialization
- **tower** (0.5): Middleware and utilities

## Testing

The project includes comprehensive tests:

- **Unit Tests**: Tests for word loading, generation logic
- **Integration Tests**: End-to-end functionality tests
- **All tests**: 6 tests covering the full stack

Run tests with:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Examples

### Generated Names (all lowercase)

- cosmic-dragon
- blazing-phoenix
- mighty-warrior
- quantum-wizard
- celestial-titan
- ancient-sentinel
- electric-thunder
- mystical-sage

## API Endpoints

### `GET /`

Generates a random cool name.

**Accept: text/html** (default)
- Returns: Beautiful HTML page with the generated name

**Accept: application/json**
- Returns: JSON object with the name

**Response Format (JSON):**
```json
{
  "name": "adjective-noun"
}
```

**Error Response:**
```json
{
  "error": "error message"
}
```

## Development

### Project Structure

The codebase is intentionally simple:
- `lib.rs`: Core name generation logic
- `main.rs`: Web server and HTTP handlers
- `tests/`: Integration tests
- Clean separation of concerns without over-engineering

### Adding More Words

Edit the text files:
1. `adjectives.txt` - Add one adjective per line
2. `nouns.txt` - Add one noun per line

Restart the application to load new words.

## Contributing

Contributions are welcome! Here are some ways you can contribute:

### Adding New Words

Want to expand the vocabulary? Submit a PR with:
- New adjectives to `adjectives.txt`
- New nouns to `nouns.txt`

Add one word per line. Make sure words are appropriate and follow the existing style.

### Code Contributions

For code changes, please ensure:
1. All tests pass (`cargo test`)
2. Code follows Rust conventions (`cargo fmt` and `cargo clippy`)
3. New features include tests
4. Update documentation as needed
