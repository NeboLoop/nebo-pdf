use crate::spec::{Page, PdfBlock, PdfMetadata, PdfSpec};
use anyhow::{Context, Result};
use lopdf::Document;
use std::path::Path;

/// Extract text from a PDF into a PdfSpec.
pub fn extract_pdf(input: &Path) -> Result<PdfSpec> {
    let doc = Document::load(input).context("failed to load PDF")?;

    let mut pages = Vec::new();
    let page_count = doc.get_pages().len();

    for page_num in 1..=page_count as u32 {
        let text = doc.extract_text(&[page_num]).unwrap_or_default();
        let paragraphs: Vec<PdfBlock> = text
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|line| PdfBlock::Paragraph {
                paragraph: line.to_string(),
                x: None,
                y: None,
            })
            .collect();

        pages.push(Page { body: paragraphs });
    }

    // Extract metadata
    let metadata = extract_metadata(&doc);

    Ok(PdfSpec {
        version: 1,
        metadata,
        page: None,
        styles: None,
        fonts: None,
        pages,
        watermark: None,
    })
}

fn extract_metadata(doc: &Document) -> Option<PdfMetadata> {
    let info_ref = doc.trailer.get(b"Info").ok()?;
    let info = doc.dereference(info_ref).ok()?;
    let dict = info.1.as_dict().ok()?;

    let title = dict
        .get(b"Title")
        .ok()
        .and_then(|v| v.as_str().ok())
        .map(|s| String::from_utf8_lossy(s).to_string());

    let creator = dict
        .get(b"Creator")
        .ok()
        .and_then(|v| v.as_str().ok())
        .map(|s| String::from_utf8_lossy(s).to_string());

    if title.is_none() && creator.is_none() {
        return None;
    }

    Some(PdfMetadata {
        title,
        creator,
        subject: None,
    })
}
