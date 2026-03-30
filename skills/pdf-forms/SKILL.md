---
name: pdf-forms
description: "PDF forms: filling existing PDF form fields, extracting text from PDFs."
license: MIT
---

# pdf +forms

> **PREREQUISITE:** Read [`../pdf/SKILL.md`](../pdf/SKILL.md) for commands and JSON spec basics.

## Form Filling

Fill an existing PDF form with values:

```bash
nebo-pdf fill template.pdf fields.json -o filled.pdf
```

`fields.json`:
```json
{
  "name": "John Doe",
  "date": "2026-01-15",
  "amount": "500.00",
  "approved": true
}
```

Keys match PDF form field names (the `T` attribute in AcroForm fields).

## Text Extraction

Extract text from an existing PDF to JSON:

```bash
nebo-pdf extract document.pdf -o output.json --pretty
```

Returns a PdfSpec with paragraphs extracted per page.

## See Also

- [pdf](../pdf/SKILL.md) — Service overview
