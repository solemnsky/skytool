## Usage
```
# Convert an altx file to a .sky:
skytool convert <file> [-o <directory>]
skytool (-h | --help)

Options:
  -h --help     Show this screen.
  -o --output   The directory in which to save files. [default: ./]
```

## Compilation

Skytool requires Rust nightly. To compile, run the following:
```
cargo build --release
```

The binary will be created as `target/release/skytool`.
