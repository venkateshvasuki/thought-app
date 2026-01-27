# Thought App

A CLI application to capture random thoughts in a structured manner so that no idea is lost.

## Overview

Thought App is a dual-mode Rust CLI application designed for capturing, organizing, and reviewing ideas. It stores thoughts in a SQLite database and can send weekly email summaries with AI-powered analysis of project ideas.

## Features

- **Categorized Thought Capture**: Organize thoughts into 5 categories (Notes, Project, Misc, Todo, Question)
- **Persistent Storage**: SQLite database with automatic tracking of reviewed status
- **AI Analysis**: Automatic analysis of project ideas using Gemini API
- **Email Summaries**: HTML-formatted weekly roundup emails via SMTP
- **Modular Design**: Feature-gated compilation separates writer and reader modes

## Architecture

```
thought-app/
├── src/
│   ├── main.rs           # Entry point with feature-gated main functions
│   ├── thought.rs        # Thought struct and email body formatting
│   ├── db_operations.rs  # SQLite CRUD operations
│   ├── errors.rs         # Custom error types
│   ├── writer_config.rs  # Writer CLI arguments & ThoughtType enum
│   ├── reader_config.rs  # Reader CLI args & config file parsing
│   ├── email.rs          # SMTP email sending
│   └── client.rs         # AI client API communication
├── Cargo.toml            # Dependencies and features
└── config.toml           # Configuration (email, AI client)
```

## Prerequisites

- Rust toolchain (rustc, cargo) - [Install Rust](https://rustup.rs/)
- SMTP email account with app password enabled
- Gemini API key (for AI analysis feature)

## Installation

1. Clone the repository
2. Set up your database path:
   ```bash
   export DB_PATH=/path/to/thought_app.db
   ```
   If not set, defaults to `thought_app.db` in the current directory.

## Configuration

Create a `config.toml` file in the project root:

```toml
[email_config]
sender_email = "your-email@provider.com"
receiver_email = "recipient@email.com"
app_password = "your-app-password"
relay = "smtp.your-provider.com"
name = "Your Name"

[ai_client_config]
bearer_token = "your-api-key"
ai_client = "Gemini"
```

**Security Note**: Keep `config.toml` private. Add it to `.gitignore` to prevent committing credentials.

### Supported AI Clients

| Client | Endpoint |
|--------|----------|
| Gemini | `https://generativelanguage.googleapis.com/v1/models/gemini-2.0-flash:generateContent` |
| OpenAI | `https://api.openai.com/v1/chat/completions` |
| Claude | `https://api.anthropic.com/v1/messages` |

*Note: Currently, only Gemini is fully implemented.*

## Usage

### Writer Mode

Capture a new thought:

```bash
cargo run --release --features writer -- --thought-type <TYPE> --content "Your thought here"
```

**Thought Types:**
- `notes` - General notes
- `project` - Project/startup ideas (triggers AI analysis in reader mode)
- `misc` - Miscellaneous thoughts
- `todo` - Tasks and reminders
- `question` - Questions to research

**Examples:**

```bash
# Add a project idea
cargo run --release --features writer -- --thought-type project -c "Build a CLI tool for tracking daily habits"

# Add a todo
cargo run --release --features writer -- --thought-type todo -c "Review pull request for auth module"

# Add a question
cargo run --release --features writer -- --thought-type question -c "How does WebSocket authentication work?"
```

### Reader Mode

Read unreviewed thoughts, get AI analysis, and send email summary:

```bash
cargo run --release --features reader
```

**Options:**
- `-c, --config <PATH>` - Path to config file (default: `config.toml`)
- `-v, --verbose` - Enable verbose output

**Example:**

```bash
cargo run --release --features reader -- --config /path/to/custom-config.toml
```

## Workflow

```
Writer Mode                          Reader Mode
    │                                    │
    ▼                                    ▼
Parse CLI Args                     Load config.toml
    │                                    │
    ▼                                    ▼
Connect to SQLite DB               Connect to SQLite DB
    │                                    │
    ▼                                    ▼
Insert Thought                     Read Unreviewed Thoughts
    │                                    │
    ▼                                    ▼
  Done                             Mark as Reviewed
                                         │
                                         ▼
                                  Filter Project Ideas
                                         │
                                         ▼
                                  Send to AI for Analysis
                                         │
                                         ▼
                                  Send Email Summary
                                         │
                                         ▼
                                       Done
```

## Database Schema

```sql
CREATE TABLE thoughts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    type TEXT NOT NULL,
    content TEXT NOT NULL,
    reviewed BOOLEAN NOT NULL DEFAULT FALSE
);
```

## Module Reference

### `thought.rs`
- `Thought` - Core data structure with id, type, content, and reviewed status
- `ThoughtsEmailBody` - Implements `IntoBody` trait for email serialization

### `db_operations.rs`
- `setup_db(db_name)` - Creates SQLite connection and initializes table
- `write_to_db(conn, args)` - Inserts a new thought
- `read(conn)` - Retrieves unreviewed thoughts and marks them reviewed

### `writer_config.rs`
- `Args` - CLI argument struct for writer mode
- `ThoughtType` - Enum for thought categories

### `reader_config.rs`
- `Args` - CLI argument struct for reader mode
- `Config` - Configuration file structure
- `EmailConfig` - SMTP email settings
- `AIClientConfig` - AI service configuration
- `AIClient` - Enum for supported AI providers

### `email.rs`
- `send_email(thoughts, config)` - Sends HTML-formatted email via SMTP

### `client.rs`
- `get_response(config, content)` - Sends project ideas to AI for analysis

### `errors.rs`
- `AppError` - Unified error enum with variants for Clap, Database, SMTP, Config, IO, and HTTP errors

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.5 | CLI argument parsing |
| rusqlite | 0.37.0 | SQLite database |
| strum/strum_macros | 0.27.2 | Enum string utilities |
| lettre | 0.11 | SMTP email |
| toml | 0.9.8 | Config file parsing |
| reqwest | 0.12 | HTTP client |
| serde | 1.0.228 | Serialization |

## Roadmap

- [ ] Full implementation for OpenAI and Claude AI clients
- [ ] Cron job integration for automatic reader execution
- [ ] Homebrew formula for easier installation
- [ ] Web interface for thought management
- [ ] Thought search and filtering capabilities

## License

This project is unlicensed. Feel free to use and modify as needed.

