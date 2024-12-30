# Liveness Check

A configurable service monitoring tool written in Rust that performs HTTP health checks on specified endpoints and sends notifications via Telegram when status changes occur.

## Features

- Configurable cron-based health checks
- Multiple target endpoint monitoring
- Telegram notifications for status changes
- In-memory state management
- Custom HTTP client with cookie support

## Configuration

Create a `config.json` file in the project root:

```json
{
  "cron": "*/20 * * * *",
  "targets": [
    {
      "url": "https://example.com",
      "name": "Example Service"
    }
  ],
  "notification": {
    "telegram": {
      "bot": "your-bot-token",
      "chat": "your-chat-id"
    }
  }
}
```

### Configuration Options

- `cron`: Schedule for health checks using cron syntax
- `targets`: Array of services to monitor
  - `url`: Endpoint URL
  - `name`: Service identifier
- `notification`: Optional notification settings
  - `telegram`: Telegram bot configuration
    - `bot`: Bot token
    - `chat`: Chat ID for notifications

## Building

```bash
cargo build --release
```

## Running

```bash
RUST_LOG=info cargo run
```

Environment Variables:
- `RUST_LOG`: Log level (trace, debug, info, warn, error)

## Notifications

The service sends notifications when:
- A service goes down
- A service recovers
- Request errors occur

## License

MIT