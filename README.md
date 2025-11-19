# Cool Names

A Rust web service that generates random cool names by combining adjectives and nouns. Built with Axum using hexagonal architecture.

## Features

- **Random Name Generation**: Combines random adjectives with random nouns (52,719+ possible combinations)
- **Content Negotiation**: Returns HTML or JSON based on the `Accept` header
- **Beautiful UI**: Modern, responsive web interface with gradient design
- **Hexagonal Architecture**: Clean separation of concerns with domain-driven design
- **Comprehensive Testing**: Unit and integration tests across all layers

## Quick Start

### Prerequisites

- Rust 1.70+ (edition 2021)
- Cargo

### Installation

```bash
git clone <repository-url>
cd cool-names
```

### Running the Application

```bash
cargo run
```

The server will start on `http://127.0.0.1:3002`

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

Without specifying the Accept header, you'll receive HTML:

```bash
curl http://127.0.0.1:3002/
```

## Architecture

This project follows **Hexagonal Architecture** (Ports and Adapters) for clean separation of concerns:

```
src/
├── domain/           # Business logic (entities and ports)
│   ├── entities.rs   # Adjective, Noun, CoolName
│   └── ports.rs      # WordRepository, NameGenerator traits
├── application/      # Use cases
│   └── name_generator_service.rs
├── infrastructure/   # External implementations
│   └── file_word_repository.rs
├── adapters/         # External interfaces
│   └── http.rs       # Axum HTTP adapter
├── lib.rs
└── main.rs           # Application entry point
```

### Layers

1. **Domain Layer**: Core business logic, no external dependencies
   - Entities: `Adjective`, `Noun`, `CoolName`
   - Ports: Trait definitions for repositories and services

2. **Application Layer**: Use cases and business workflows
   - `NameGeneratorService`: Orchestrates name generation

3. **Infrastructure Layer**: Technical implementations
   - `FileWordRepository`: Reads and caches words from text files

4. **Adapters Layer**: External interfaces
   - HTTP adapter with Axum for REST API

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

- **axum** (0.7): Modern web framework
- **tokio** (1.x): Async runtime
- **rand** (0.8): Random number generation
- **thiserror** (1.0): Error handling
- **serde** (1.0): Serialization/deserialization
- **tower** (0.4): Middleware and utilities

## Testing

The project includes comprehensive tests:

- **Unit Tests**: 13 tests covering all business logic
- **Integration Tests**: 2 tests validating end-to-end functionality
- **Coverage**: Domain, application, infrastructure, and adapter layers

Run tests with:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

## Examples

### Generated Names

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

The codebase follows Rust best practices:
- Each module has comprehensive tests
- Clear separation between layers
- Dependency injection through traits
- No circular dependencies

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
