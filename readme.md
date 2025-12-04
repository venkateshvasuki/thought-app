# Thought App

An Application to capture random thoughts in a structured manner so that no idea is lost.

This application uses Rust for it's backend and writes data to a .db file. The reader will be used to read the db and
export and email.

To do:

- Add AI capability for specific types to add more context to ideas.
- A cron job setting so that the reader runs automatically
- Publish homebrew for easier installation.

Steps to run:

Make sure rustc and cargo are installed.

1. export DB_PATH=path/to/.db/file
2. Configure your email for smtp and obtain the app password.
3. create a cargo.toml file with following fields:

    - sender_email
    - receiver_email
    - app_password
    - relay
    - name
4. Run cargo run --release --features writer -- --thought-type thought-type --content "content"
5. Run cargo run --release --features reader


