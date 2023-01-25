### Install cargo watch
```
cargo install cargo-watch
cargo watch -x run ##execute with watch
```
### Install cargo documentation
```
cargo doc
cargo doc --open
```

## Axum Features
```
cargo add axum -F headers
```
## Read .env
```
cargo add dotenvy
cargo add dotenvy_macro
```

## Sea-ORM cli install
```
cargo install sea-orm-cli #install orm-clir

# List all available commands
sea-orm-cli -h

# List all subcommands available in `generate` command
sea-orm-cli generate -h

# Show how to use `generate entity` subcommand
sea-orm-cli generate entity -h

sea-orm-cli generate entity -o src/models # generate entity
```