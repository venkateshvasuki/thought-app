# Testing Guide for Thought App

This document describes the testing architecture, mock patterns, and how to run tests for the thought-app Rust CLI.

## Testing Overview

The thought-app includes comprehensive unit and integration tests across all core modules. The testing strategy minimizes external dependencies through:

- **In-memory SQLite databases** for database tests
- **Trait-based mocking** for email transport
- **WireMock HTTP server** for API client tests
- **Temporary files** for configuration tests

Total test count: **39+ tests** covering all major functionality.

## Test Architecture

### 1. Pure Logic Tests (No Mocking Required)

Tests for pure functions that don't interact with external systems:

- **writer_config.rs** (8 tests):
  - ThoughtType display and parsing
  - Case-insensitive string conversion
  - SQL serialization/deserialization
  - Args getters

- **thought.rs** (6 tests):
  - Thought constructor and getters
  - ThoughtsEmailBody HTML generation
  - Empty thoughts handling
  - Multiple thoughts formatting

### 2. Database Tests (In-Memory SQLite)

Tests using `:memory:` SQLite databases - no mocking library needed:

- **db_operations.rs** (6 tests):
  - `setup_db` creates table correctly
  - `write_to_db` inserts thoughts
  - `read_from_db` returns only unreviewed thoughts
  - `read` marks thoughts as reviewed
  - `update_db` updates multiple thoughts
  - Empty database handling

**Example:**
```rust
fn create_in_memory_db() -> Connection {
    setup_db(":memory:").unwrap()
}

#[test]
fn test_write_to_db_inserts_thought() {
    let conn = create_in_memory_db();
    let args = Args {
        thought_type: ThoughtType::Notes,
        content: "Test thought".to_string(),
    };
    write_to_db(&conn, &args).unwrap();
    // Verify...
}
```

### 3. Configuration Tests (Temporary Files)

Tests using `tempfile` crate for config file I/O:

- **reader_config.rs** (9 tests):
  - Parse valid/invalid TOML
  - Load config from file using `tempfile`
  - Bearer token formatting for different AI clients
  - Email config getters
  - File not found errors

**Example:**
```rust
use tempfile::NamedTempFile;

#[test]
fn test_load_config_from_file() {
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(toml_content.as_bytes()).unwrap();

    let args = Args {
        config: temp_file.path().to_path_buf(),
        verbose: true,
    };

    let config = args.config().unwrap();
    // Verify...
}
```

### 4. Email Tests (Trait-Based Mocking)

The email module uses a trait abstraction for testability:

```rust
pub trait EmailTransport {
    fn send(&self, email: &Message) -> Result<(), String>;
}

impl EmailTransport for SmtpTransport {
    fn send(&self, email: &Message) -> Result<(), String> {
        // Real SMTP implementation
    }
}
```

**Mock Implementation:**
```rust
struct MockEmailTransport {
    should_fail: bool,
    sent_emails: RefCell<Vec<String>>,
}

impl EmailTransport for MockEmailTransport {
    fn send(&self, _email: &Message) -> Result<(), String> {
        if self.should_fail {
            Err("Mock SMTP error".to_string())
        } else {
            self.sent_emails.borrow_mut().push("sent".to_string());
            Ok(())
        }
    }
}
```

- **email.rs** (5 tests):
  - Send email success
  - Send email failure
  - Empty thoughts list
  - Invalid email addresses
  - Multiple thoughts

### 5. HTTP Client Tests (WireMock)

Tests using `wiremock` crate to create mock HTTP server:

- **client.rs** (5 async tests):
  - `get_request` builds correct headers
  - `send_request` handles 200 responses
  - `get_response` end-to-end integration
  - Network error handling
  - API key header verification

**Example:**
```rust
#[tokio::test]
async fn test_send_request_handles_200_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200).set_body_string("success response"))
        .mount(&mock_server)
        .await;

    let client = Client::new();
    let request = client
        .post(format!("{}/test", &mock_server.uri()))
        .build()
        .unwrap();

    let response = send_request(&client, request).unwrap();
    assert_eq!(response, "success response");
}
```

### 6. Integration Tests

Feature-gated integration tests in the `tests/` directory:

- **writer_integration.rs** (3 tests):
  - Write and verify end-to-end
  - Write multiple thoughts
  - Write different thought types

- **reader_integration.rs** (5 tests):
  - Read end-to-end
  - Read twice returns empty
  - Read ignores already reviewed
  - Read empty database
  - Read multiple thought types

## Running Tests

### Test by Feature

Since the app uses feature gates, tests must be run separately for each feature:

```bash
# Test writer feature
cargo test --features writer

# Test reader feature
cargo test --features reader

# Test library code only (no feature-specific code)
cargo test --lib
```

### Test Specific Modules

```bash
# Test only writer_config module
cargo test --features writer writer_config

# Test only database operations
cargo test --features writer db_operations

# Test only email module
cargo test --features reader email

# Test only client module
cargo test --features reader client
```

### Run All Tests

To run all tests across both features:

```bash
cargo test --features writer && cargo test --features reader
```

### Verbose Output

For detailed test output:

```bash
cargo test --features writer -- --nocapture
```

### Run Integration Tests Only

```bash
cargo test --features writer --test writer_integration
cargo test --features reader --test reader_integration
```

## Test Coverage Summary

| Module | Unit Tests | Integration Tests | Total |
|--------|------------|-------------------|-------|
| writer_config.rs | 8 | - | 8 |
| thought.rs | 6 | - | 6 |
| db_operations.rs | 6 | - | 6 |
| reader_config.rs | 9 | - | 9 |
| email.rs | 5 | - | 5 |
| client.rs | 5 | - | 5 |
| Integration | - | 8 | 8 |
| **Total** | **39** | **8** | **47** |

## Mock Patterns Reference

### Pattern 1: In-Memory Database
**Use when:** Testing database operations
**Setup:** `setup_db(":memory:")`
**Cleanup:** Automatic (connection dropped)

### Pattern 2: Temporary Files
**Use when:** Testing file I/O
**Setup:** `NamedTempFile::new()`
**Cleanup:** Automatic (file deleted on drop)

### Pattern 3: Trait-Based Mocks
**Use when:** Need to mock complex behavior
**Setup:** Define trait, implement for both real and mock types
**Example:** `EmailTransport` trait

### Pattern 4: HTTP Mock Server
**Use when:** Testing HTTP clients
**Setup:** `MockServer::start().await`
**Cleanup:** Automatic (server stopped on drop)

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Test

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run writer tests
        run: cargo test --features writer
      - name: Run reader tests
        run: cargo test --features reader
```

### Test Coverage with Tarpaulin

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --features writer --out Html
cargo tarpaulin --features reader --out Html
```

## Writing New Tests

### Adding a Unit Test

1. Add `#[cfg(test)]` module at the end of your source file
2. Import necessary items: `use super::*;`
3. Write test functions with `#[test]` attribute
4. Use assertions: `assert!`, `assert_eq!`, `assert!(result.is_ok())`

Example:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_function() {
        let result = my_function();
        assert_eq!(result, expected_value);
    }
}
```

### Adding an Async Test

For tests using `wiremock`:

```rust
#[tokio::test]
async fn test_async_function() {
    let mock_server = MockServer::start().await;
    // ... test logic
}
```

### Adding an Integration Test

1. Create a new file in `tests/` directory
2. Add feature gate: `#[cfg(feature = "writer")]`
3. Import from crate: `use thought::module::function;`
4. Write tests with `#[test]` attribute

## Troubleshooting

### Issue: Tests fail with "feature not enabled"
**Solution:** Run with correct feature flag: `cargo test --features writer`

### Issue: Async tests fail to compile
**Solution:** Ensure `tokio` dev-dependency is present with `macros` feature

### Issue: Database tests interfere with each other
**Solution:** Use `:memory:` databases - each test gets isolated DB

### Issue: Mock server port conflicts
**Solution:** WireMock automatically assigns unique ports per test

## Performance

Test execution time: **< 5 seconds** for all 47 tests

Fast execution is achieved through:
- In-memory databases (no disk I/O)
- Parallel test execution (Rust default)
- Lightweight mocks (no heavy frameworks)
- Feature isolation (only relevant tests run)

## Best Practices

1. **Use in-memory databases** for DB tests - no cleanup needed
2. **Keep tests isolated** - each test should be independent
3. **Test error cases** - verify error handling works correctly
4. **Use descriptive names** - `test_write_to_db_inserts_thought` vs `test_write`
5. **Avoid real external services** - always use mocks for email, HTTP, etc.
6. **Feature-gate integration tests** - respect feature boundaries
7. **Clean test data** - use temporary files, in-memory DBs for automatic cleanup

## Git Workflow

### Branching Strategy

**IMPORTANT: Always create a new branch for changes. Never commit directly to `master` or `main`.**

#### Creating a Feature Branch

```bash
# Create and checkout a new branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/bug-description
```

#### Branch Naming Conventions

- **Features**: `feature/description` (e.g., `feature/add-authentication`)
- **Bug fixes**: `fix/description` (e.g., `fix/email-sending-error`)
- **Tests**: `test/description` (e.g., `test/add-unit-tests`)
- **Documentation**: `docs/description` (e.g., `docs/update-readme`)
- **Refactoring**: `refactor/description` (e.g., `refactor/extract-config-parser`)

#### Committing Changes

```bash
# Stage specific files
git add src/file1.rs src/file2.rs

# Or stage all changes (be careful!)
git add .

# Commit with descriptive message
git commit -m "Add feature: description of changes

- Bullet point 1
- Bullet point 2
- Bullet point 3

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

#### Pushing to GitHub

```bash
# First time pushing the branch
git push -u origin feature/your-feature-name

# Subsequent pushes
git push
```

#### Creating Pull Requests

After pushing your branch, GitHub will provide a URL to create a pull request:

```
https://github.com/venkateshvasuki/thought-app/pull/new/feature/your-feature-name
```

**Pull Request Checklist:**
1. ✅ All tests pass (`cargo test --features writer && cargo test --features reader`)
2. ✅ Code follows project conventions
3. ✅ Descriptive PR title and description
4. ✅ Related issues linked (if applicable)
5. ✅ Documentation updated (if needed)

#### Merging Strategy

- Use **"Squash and merge"** for feature branches to keep history clean
- Use **"Merge commit"** for important milestones
- Delete branch after merging

#### Example Workflow

```bash
# 1. Start from master
git checkout master
git pull origin master

# 2. Create feature branch
git checkout -b feature/add-user-settings

# 3. Make changes and test
cargo test --features writer
cargo test --features reader

# 4. Commit changes
git add src/settings.rs tests/settings_test.rs
git commit -m "Add user settings configuration

- Add Settings struct
- Implement TOML parsing
- Add unit tests
- Update documentation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"

# 5. Push to GitHub
git push -u origin feature/add-user-settings

# 6. Create PR on GitHub and request review

# 7. After approval, merge PR and delete remote branch

# 8. Clean up local branch
git checkout master
git pull origin master
git branch -d feature/add-user-settings
```

#### Syncing with Master

If master has changed while working on your branch:

```bash
# Option 1: Rebase (cleaner history)
git checkout feature/your-branch
git fetch origin
git rebase origin/master

# Option 2: Merge (preserves history)
git checkout feature/your-branch
git merge origin/master
```

### Pre-commit Checklist

Before committing:
- [ ] Run tests: `cargo test --features writer && cargo test --features reader`
- [ ] Check formatting: `cargo fmt --check`
- [ ] Run clippy: `cargo clippy --features writer && cargo clippy --features reader`
- [ ] Update documentation if needed
- [ ] Review changes: `git diff`

### Commit Message Guidelines

**Format:**
```
<type>: <short summary> (50 chars or less)

<detailed description with bullet points>
- What changed
- Why it changed
- Any breaking changes

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `test`: Adding or updating tests
- `refactor`: Code refactoring
- `docs`: Documentation changes
- `chore`: Maintenance tasks

## References

- [Rust Book - Writing Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [WireMock Documentation](https://docs.rs/wiremock/)
- [Tokio Testing](https://tokio.rs/tokio/topics/testing)
- [Tempfile Crate](https://docs.rs/tempfile/)
- [SQLite In-Memory Databases](https://www.sqlite.org/inmemorydb.html)

## Support

For questions or issues with tests:
1. Check test output with `--nocapture` flag
2. Review this guide's mock patterns
3. Examine existing tests for similar scenarios
4. Open an issue on the project repository
