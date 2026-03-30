use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root PDF specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfSpec {
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<PdfMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<PageConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<PdfStyles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fonts: Option<HashMap<String, String>>,
    pub pages: Vec<Page>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watermark: Option<Watermark>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub margin: Option<MarginConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarginConfig {
    #[serde(default)]
    pub top: Option<f64>,
    #[serde(default)]
    pub bottom: Option<f64>,
    #[serde(default)]
    pub left: Option<f64>,
    #[serde(default)]
    pub right: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfStyles {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub body: Vec<PdfBlock>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PdfBlock {
    Heading {
        heading: u8,
        text: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
    },
    Paragraph {
        paragraph: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
    },
    Table {
        table: Vec<Vec<String>>,
        #[serde(default, rename = "header-rows", skip_serializing_if = "Option::is_none")]
        header_rows: Option<u32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
    },
    Image {
        image: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        x: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        y: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        width: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        height: Option<f64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watermark {
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rotate: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
}

impl PageConfig {
    /// Returns (width_pt, height_pt) for the configured page size.
    pub fn dimensions_pt(&self) -> (f64, f64) {
        match self.size.as_deref() {
            Some("letter") | None => (612.0, 792.0),
            Some("a4") => (595.28, 841.89),
            Some("legal") => (612.0, 1008.0),
            _ => (612.0, 792.0),
        }
    }

    pub fn margins_pt(&self) -> (f64, f64, f64, f64) {
        let m = self.margin.as_ref();
        let top = m.and_then(|m| m.top).unwrap_or(1.0) * 72.0;
        let bottom = m.and_then(|m| m.bottom).unwrap_or(1.0) * 72.0;
        let left = m.and_then(|m| m.left).unwrap_or(1.0) * 72.0;
        let right = m.and_then(|m| m.right).unwrap_or(1.0) * 72.0;
        (top, bottom, left, right)
    }
}

/// Fields for PDF form filling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormFields {
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}
