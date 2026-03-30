---
name: nebo-pdf
description: "PDF generation, form filling, and text extraction binary. Creates PDFs from JSON specs, fills existing PDF forms, and extracts text. Skills access the binary via $NEBO_PDF_BIN."
version: "0.1.0"
license: MIT
---

# nebo-pdf — PDF Generation & Manipulation

Compiled Rust binary for creating PDFs from JSON specs, filling PDF form fields, and extracting text from existing PDFs. Skills access the binary path via `$NEBO_PDF_BIN`.

## Services

| Skill | Capability |
|-------|-----------|
| `pdf` | Create, fill, extract, and validate PDF documents (.pdf) |

## Helpers

| Skill | What it covers |
|-------|---------------|
| `pdf-elements` | Tables, images, positioned elements, watermarks |
| `pdf-forms` | Form filling, text extraction |
