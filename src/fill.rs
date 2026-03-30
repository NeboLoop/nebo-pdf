use crate::spec::FormFields;
use anyhow::{Context, Result};
use lopdf::Document;
use std::path::Path;

/// Fill a PDF form with field values.
pub fn fill_pdf(input: &Path, fields: &FormFields, output: &Path) -> Result<()> {
    let mut doc = Document::load(input).context("failed to load PDF")?;

    // Get the AcroForm dictionary
    if let Ok(catalog) = doc.catalog().cloned() {
        if let Ok(acroform_ref) = catalog.get(b"AcroForm") {
            if let Ok(acroform) = doc.dereference(acroform_ref) {
                if let Ok(acroform_dict) = acroform.1.as_dict() {
                    if let Ok(field_refs) = acroform_dict.get(b"Fields") {
                        if let Ok(fields_array) = doc.dereference(field_refs) {
                            if let Ok(arr) = fields_array.1.as_array() {
                                let field_ids: Vec<_> = arr.iter().filter_map(|r| {
                                    if let lopdf::Object::Reference(id) = r {
                                        Some(*id)
                                    } else {
                                        None
                                    }
                                }).collect();

                                for field_id in field_ids {
                                    set_field_value(&mut doc, field_id, &fields.fields);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    doc.save(output).context("failed to save PDF")?;
    Ok(())
}

fn set_field_value(
    doc: &mut Document,
    field_id: lopdf::ObjectId,
    values: &std::collections::HashMap<String, serde_json::Value>,
) {
    if let Ok(field) = doc.get_object(field_id).cloned() {
        if let Ok(dict) = field.as_dict() {
            // Get field name
            let name = dict
                .get(b"T")
                .ok()
                .and_then(|v| v.as_str().ok())
                .map(|s| String::from_utf8_lossy(s).to_string());

            if let Some(ref field_name) = name {
                if let Some(new_value) = values.get(field_name) {
                    let value_str = match new_value {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        _ => new_value.to_string(),
                    };

                    if let Ok(field_dict) = doc.get_object_mut(field_id) {
                        if let Ok(dict) = field_dict.as_dict_mut() {
                            dict.set(b"V", lopdf::Object::string_literal(value_str));
                        }
                    }
                }
            }

            // Recurse into child fields (Kids array)
            if let Ok(kids) = dict.get(b"Kids") {
                if let Ok(kids_arr) = kids.as_array() {
                    let kid_ids: Vec<_> = kids_arr.iter().filter_map(|r| {
                        if let lopdf::Object::Reference(id) = r {
                            Some(*id)
                        } else {
                            None
                        }
                    }).collect();

                    for kid_id in kid_ids {
                        set_field_value(doc, kid_id, values);
                    }
                }
            }
        }
    }
}
