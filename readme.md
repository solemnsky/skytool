A tool for working with Solemnsky's environment files.

## Usage
```
  skytool convert <file>  # Convert an altx file to .sky
  skytool (-h | --help)

Options:
  -h --help     Show this screen.
```

## Compilation

Skytool requires Rust nightly. To compile, run the following:
```
cargo build --release
```

The binary will be created as `target/release/skytool`.
