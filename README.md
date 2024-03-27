# Printer Server
Printer server written in pure rust

## All the available routes
- `localhost:4590/printer-names` for the list of available printers by name
- `localhost:4590/print/[printer-name]` to print the document with the specified `printer-name`

## Build instruction
### For Windows
```
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### For Linux
```
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

> Required rust setted. For instruction on how to setup [rust](https://www.rust-lang.org/learn/get-started)

