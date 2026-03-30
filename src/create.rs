use crate::spec::{PdfBlock, PdfSpec, Watermark};
use anyhow::Result;
use printpdf::*;
use std::io::{BufWriter, Write};

/// Create a PDF from a spec.
pub fn create_pdf<W: Write>(spec: &PdfSpec, writer: W, assets_dir: Option<&std::path::Path>) -> Result<()> {
    let page_config = spec.page.clone().unwrap_or(crate::spec::PageConfig {
        size: Some("letter".to_string()),
        margin: None,
    });
    let (page_w_pt, page_h_pt) = page_config.dimensions_pt();
    let (margin_top, margin_bottom, margin_left, margin_right) = page_config.margins_pt();

    let default_font_name = spec
        .styles
        .as_ref()
        .and_then(|s| s.font.as_deref())
        .unwrap_or("Helvetica");
    let default_size = spec
        .styles
        .as_ref()
        .and_then(|s| s.size)
        .unwrap_or(12.0);

    let page_w = Mm::from(Pt(page_w_pt as f32));
    let page_h = Mm::from(Pt(page_h_pt as f32));

    let (doc, first_page, first_layer) = PdfDocument::new(
        spec.metadata
            .as_ref()
            .and_then(|m| m.title.as_deref())
            .unwrap_or("Document"),
        page_w,
        page_h,
        "Layer 1",
    );

    // Get built-in font
    let font = doc.add_builtin_font(resolve_builtin_font(default_font_name))?;
    let bold_font = doc.add_builtin_font(resolve_builtin_font_bold(default_font_name))?;

    let usable_width_pt = page_w_pt - margin_left - margin_right;

    // Render pages
    let mut current_page = first_page;
    let mut current_layer_ref = first_layer;

    for (page_idx, page) in spec.pages.iter().enumerate() {
        if page_idx > 0 {
            let (new_page, new_layer) = doc.add_page(page_w, page_h, "Layer 1");
            current_page = new_page;
            current_layer_ref = new_layer;
        }

        let layer = doc.get_page(current_page).get_layer(current_layer_ref);
        let mut cursor_y = page_h_pt - margin_top;

        for block in &page.body {
            match block {
                PdfBlock::Heading { heading, text, x, y } => {
                    let size = heading_size(*heading, default_size);
                    let draw_x = x.map(|v| v * 72.0).unwrap_or(margin_left);
                    let draw_y = y.map(|v| page_h_pt - v * 72.0).unwrap_or(cursor_y);

                    layer.use_text(text, size as f32, Mm::from(Pt(draw_x as f32)), Mm::from(Pt(draw_y as f32)), &bold_font);
                    cursor_y = draw_y - size - 6.0;
                }
                PdfBlock::Paragraph { paragraph, x, y } => {
                    let draw_x = x.map(|v| v * 72.0).unwrap_or(margin_left);
                    let draw_y = y.map(|v| page_h_pt - v * 72.0).unwrap_or(cursor_y);

                    // Simple word wrap
                    let lines = wrap_text(paragraph, default_size, usable_width_pt);
                    for line in &lines {
                        layer.use_text(line, default_size as f32, Mm::from(Pt(draw_x as f32)), Mm::from(Pt(draw_y as f32)), &font);
                        cursor_y -= default_size + 4.0;
                    }
                    if lines.is_empty() {
                        cursor_y -= default_size + 4.0;
                    }
                    cursor_y -= 4.0; // paragraph spacing
                }
                PdfBlock::Table { table, header_rows, x, y } => {
                    let draw_x = x.map(|v| v * 72.0).unwrap_or(margin_left);
                    let draw_y = y.map(|v| page_h_pt - v * 72.0).unwrap_or(cursor_y);

                    if table.is_empty() {
                        continue;
                    }
                    let num_cols = table[0].len().max(1);
                    let col_width = usable_width_pt / num_cols as f64;
                    let row_height = default_size + 8.0;
                    let header_count = header_rows.unwrap_or(0) as usize;

                    let mut ty = draw_y;
                    for (ri, row) in table.iter().enumerate() {
                        let use_font = if ri < header_count { &bold_font } else { &font };
                        for (ci, cell) in row.iter().enumerate() {
                            let cx = draw_x + ci as f64 * col_width + 4.0;
                            layer.use_text(cell, default_size as f32, Mm::from(Pt(cx as f32)), Mm::from(Pt(ty as f32)), use_font);
                        }
                        ty -= row_height;
                    }

                    // Draw table grid lines
                    let table_height = table.len() as f64 * row_height;
                    let table_width = num_cols as f64 * col_width;

                    let line_color = Color::Rgb(Rgb::new(0.7, 0.7, 0.7, None));
                    layer.set_outline_color(line_color);
                    layer.set_outline_thickness(0.5);

                    // Horizontal lines
                    for i in 0..=table.len() {
                        let ly = draw_y + row_height * 0.7 - i as f64 * row_height;
                        let points = vec![
                            (Point::new(Mm::from(Pt(draw_x as f32)), Mm::from(Pt(ly as f32))), false),
                            (Point::new(Mm::from(Pt((draw_x + table_width) as f32)), Mm::from(Pt(ly as f32))), false),
                        ];
                        layer.add_line(Line {
                            points,
                            is_closed: false,
                        });
                    }

                    // Vertical lines
                    let top_y = draw_y + row_height * 0.7;
                    let bottom_y = top_y - table_height;
                    for i in 0..=num_cols {
                        let lx = draw_x + i as f64 * col_width;
                        let points = vec![
                            (Point::new(Mm::from(Pt(lx as f32)), Mm::from(Pt(top_y as f32))), false),
                            (Point::new(Mm::from(Pt(lx as f32)), Mm::from(Pt(bottom_y as f32))), false),
                        ];
                        layer.add_line(Line {
                            points,
                            is_closed: false,
                        });
                    }

                    cursor_y = draw_y - table_height - 8.0;
                }
                PdfBlock::Image { image, x, y, width, height } => {
                    let draw_x = x.map(|v| v * 72.0).unwrap_or(margin_left);
                    let draw_y = y.map(|v| page_h_pt - v * 72.0).unwrap_or(cursor_y);

                    if let Some(ref dir) = assets_dir {
                        let path = dir.join(image);
                        if let Ok(data) = std::fs::read(&path) {
                            let w_pt = width.unwrap_or(2.0) * 72.0;
                            let h_pt = height.unwrap_or(2.0) * 72.0;

                            // Determine image type from extension
                            let ext = image.rsplit('.').next().unwrap_or("png").to_lowercase();
                            let image_obj = ImageXObject {
                                width: Px((w_pt * 96.0 / 72.0) as usize),
                                height: Px((h_pt * 96.0 / 72.0) as usize),
                                color_space: ColorSpace::Rgb,
                                bits_per_component: ColorBits::Bit8,
                                interpolate: true,
                                image_data: data,
                                image_filter: None,
                                clipping_bbox: None,
                                smask: None,
                            };

                            Image::from(image_obj).add_to_layer(
                                layer.clone(),
                                ImageTransform {
                                    translate_x: Some(Mm::from(Pt(draw_x as f32))),
                                    translate_y: Some(Mm::from(Pt((draw_y - h_pt) as f32))),
                                    scale_x: Some((w_pt / 96.0) as f32),
                                    scale_y: Some((h_pt / 96.0) as f32),
                                    ..Default::default()
                                },
                            );

                            cursor_y = draw_y - h_pt - 8.0;
                        }
                    }
                }
            }
        }

        // Watermark
        if let Some(ref wm) = spec.watermark {
            render_watermark(&layer, wm, page_w_pt, page_h_pt, &font);
        }
    }

    let mut buf = BufWriter::new(writer);
    doc.save(&mut buf)?;
    Ok(())
}

fn render_watermark(
    layer: &PdfLayerReference,
    wm: &Watermark,
    page_w: f64,
    page_h: f64,
    font: &IndirectFontRef,
) {
    let size = wm.size.unwrap_or(72.0);
    let color_hex = wm.color.as_deref().unwrap_or("CCCCCC");
    let r = u8::from_str_radix(&color_hex[0..2], 16).unwrap_or(200) as f32 / 255.0;
    let g = u8::from_str_radix(&color_hex[2..4], 16).unwrap_or(200) as f32 / 255.0;
    let b = u8::from_str_radix(&color_hex[4..6], 16).unwrap_or(200) as f32 / 255.0;

    layer.set_fill_color(Color::Rgb(Rgb::new(r, g, b, None)));

    // Center the watermark on the page
    let cx = page_w / 2.0;
    let cy = page_h / 2.0;
    layer.use_text(
        &wm.text,
        size as f32,
        Mm::from(Pt(cx as f32)),
        Mm::from(Pt(cy as f32)),
        font,
    );
}

fn heading_size(level: u8, default_size: f64) -> f64 {
    match level {
        1 => default_size * 2.0,
        2 => default_size * 1.5,
        3 => default_size * 1.25,
        4 => default_size * 1.1,
        _ => default_size,
    }
}

fn wrap_text(text: &str, font_size: f64, max_width: f64) -> Vec<String> {
    // Approximate: assume avg char width ~ 0.5 * font_size
    let char_width = font_size * 0.5;
    let chars_per_line = (max_width / char_width) as usize;
    if chars_per_line == 0 {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.is_empty() {
            current_line = word.to_string();
        } else if current_line.len() + 1 + word.len() <= chars_per_line {
            current_line.push(' ');
            current_line.push_str(word);
        } else {
            lines.push(current_line);
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

fn resolve_builtin_font(name: &str) -> BuiltinFont {
    match name.to_lowercase().as_str() {
        "times" | "times-roman" | "times new roman" => BuiltinFont::TimesRoman,
        "courier" | "courier new" => BuiltinFont::Courier,
        _ => BuiltinFont::Helvetica,
    }
}

fn resolve_builtin_font_bold(name: &str) -> BuiltinFont {
    match name.to_lowercase().as_str() {
        "times" | "times-roman" | "times new roman" => BuiltinFont::TimesBold,
        "courier" | "courier new" => BuiltinFont::CourierBold,
        _ => BuiltinFont::HelveticaBold,
    }
}
