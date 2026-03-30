---
name: pdf-elements
description: "PDF elements: tables, images, positioned elements, watermarks."
license: MIT
triggers:
  - pdf table
  - pdf image
  - watermark
  - pdf watermark
---

# pdf +elements

> **PREREQUISITE:** Read [`../pdf/SKILL.md`](../pdf/SKILL.md) for commands and JSON spec basics.

## Table

```json
{
  "table": [
    ["Item", "Qty", "Price", "Total"],
    ["Widget A", "10", "$25", "$250"],
    ["Widget B", "5", "$50", "$250"]
  ],
  "header-rows": 1
}
```

Header rows render in bold. Table draws grid lines automatically.

## Image

```json
{ "image": "logo.png", "width": 2, "height": 1 }
{ "image": "chart.png", "x": 5, "y": 2, "width": 3, "height": 2 }
```

- Width/height in inches
- Optional `x`, `y` for absolute positioning (inches from left/top)
- Images loaded from `--assets` directory

## Positioned Elements

Omit `x`/`y` for flow mode (content stacks top-to-bottom). Include them for absolute positioning:

```json
{ "paragraph": "This flows normally" },
{ "paragraph": "This is positioned", "x": 5, "y": 2 }
```

## Watermark

```json
"watermark": {
  "text": "DRAFT",
  "color": "CCCCCC",
  "rotate": 45,
  "size": 72
}
```

Applied to every page.

## See Also

- [pdf](../pdf/SKILL.md) — Service overview
