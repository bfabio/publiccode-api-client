# publiccode-api-client

Command-line client for the [Developers Italia API](https://api.developers.italia.it).

## Prerequisites

* Rust stable toolchain
* Environment variable `API_BEARER_TOKEN` set with your API token

## Installation

Clone the repo and install:

```bash
git clone https://github.com/your-org/publiccode-api-client.git
cd publiccode-api-client
cargo install --path .
```

Or build locally:

```bash
cargo build --release
```

The built binary will be located at `target/release/publiccode_api_client`.

## Usage

```
publiccode_api_client <SUBCOMMAND> [OPTIONS]
```

Available subcommands:

| Subcommand                          | Description                       |
| ----------------------------------- | --------------------------------- |
| `create-publisher <JSON>`           | Create a new publisher            |
| `create-software <JSON>`            | Create a new software entity      |
| `update-software <ID> <JSON>`       | Update a software entity          |
| `list-publishers [--code-hostings]` | List all publishers               |
| `list-software`                     | List all software entities        |
| `show-publisher <ID>`               | Show details of a publisher       |
| `show-software <ID>`                | Show details of a software entity |
| `logs`                              | Show current day's logs           |

### Examples

Create a publisher:

```bash
API_BEARER_TOKEN=xxx api_cli create-publisher '{"description":"Ministry of Truth","email":"techpub@example.com"}'
```

List publishers with code hostings:

```bash
api_cli list-publishers --code-hostings
```

Show a software by ID:

```bash
api_cli show-software af6056fc-b2b2-4d31-9961-c9bd94e32bd4
```

## License

[GPL-3.0](LICENSE)

