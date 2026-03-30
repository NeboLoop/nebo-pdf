---
name: nebo-pdf
description: "PDF generation, form filling, and text extraction binary. Creates PDFs from JSON specs, fills existing PDF forms, and extracts text. Skills depend on this plugin to get the binary via $NEBO_PDF_BIN."
version: "0.1.0"
license: MIT
---

# nebo-pdf — PDF Generation & Manipulation

Compiled Rust binary using `printpdf` and `lopdf` for creating PDFs from JSON specs, filling PDF forms, and extracting text from existing PDFs.

## Env Var

Skills access the binary path via `$NEBO_PDF_BIN`.

## Commands

| Command | Description |
|---------|-------------|
| `nebo-pdf create` | JSON spec to PDF |
| `nebo-pdf fill` | Fill existing PDF form fields |
| `nebo-pdf extract` | Extract text from PDF to JSON |
| `nebo-pdf validate` | Validate a JSON spec |
