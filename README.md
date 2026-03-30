# nebo-pdf

Nebo plugin for PDF generation, form filling, and text extraction. Compiled Rust binary using `printpdf` and `lopdf` — no runtime dependencies.

## Commands

```bash
nebo-pdf create spec.json -o output.pdf [--assets <dir>]
nebo-pdf fill input.pdf fields.json -o output.pdf
nebo-pdf extract input.pdf -o spec.json [--pretty]
nebo-pdf validate spec.json
```

## Building

```bash
cargo build --release
```

Cross-compile for all platforms:

```bash
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
cross build --release --target aarch64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-gnu
cross build --release --target x86_64-pc-windows-gnu
```

## License

MIT
