# rust-hash-tool

CLI tool to generate SHA256 and SHA512 hashes.

## Install

```bash
cargo build --release
```

## Usage

```bash
# SHA256 (default)
cargo run -- -i "hello world"

# SHA512
cargo run -- -i "hello world" -a sha512

# Help
cargo run -- --help
```

## Options

- `-i, --input` - Text to hash (required)
- `-a, --algorithm` - Algorithm: sha256 or sha512 (default: sha256)
