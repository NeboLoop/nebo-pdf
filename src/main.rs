mod create;
mod extract;
mod fill;
mod spec;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "nebo-pdf", version, about = "Generate and manipulate PDF documents from JSON specs")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a PDF from a JSON spec
    Create {
        /// Path to the JSON spec file, or "-" for stdin
        spec: String,
        /// Output PDF file path
        #[arg(short, long)]
        output: String,
        /// Directory containing image assets
        #[arg(long)]
        assets: Option<PathBuf>,
    },
    /// Fill a PDF form with field values
    Fill {
        /// Path to the input PDF file
        input: String,
        /// Path to the fields JSON file
        fields: String,
        /// Output PDF file path
        #[arg(short, long)]
        output: String,
    },
    /// Extract text from a PDF to JSON
    Extract {
        /// Path to the input PDF file
        input: String,
        /// Output JSON file path, or "-" for stdout
        #[arg(short, long)]
        output: String,
        /// Directory to extract image assets to
        #[arg(long)]
        assets: Option<PathBuf>,
        /// Pretty-print JSON output
        #[arg(long)]
        pretty: bool,
    },
    /// Validate a JSON spec
    Validate {
        /// Path to the JSON spec file
        spec: String,
    },
    /// Print version information
    Version,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version => {
            println!("nebo-pdf {}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Create {
            spec,
            output,
            assets,
        } => {
            let json = read_input(&spec)?;
            let pdf_spec: spec::PdfSpec =
                serde_json::from_str(&json).context("failed to parse JSON spec")?;

            let assets_dir = assets.or_else(|| {
                if spec != "-" {
                    PathBuf::from(&spec).parent().map(|p| p.to_path_buf())
                } else {
                    None
                }
            });

            let mut buf = Vec::new();
            create::create_pdf(&pdf_spec, &mut buf, assets_dir.as_deref())?;
            std::fs::write(&output, &buf)
                .with_context(|| format!("failed to write {output}"))?;
            eprintln!("Created: {output}");
        }
        Commands::Fill {
            input,
            fields,
            output,
        } => {
            let fields_json = std::fs::read_to_string(&fields)
                .with_context(|| format!("failed to read {fields}"))?;
            let form_fields: spec::FormFields =
                serde_json::from_str(&fields_json).context("failed to parse fields JSON")?;

            fill::fill_pdf(
                &PathBuf::from(&input),
                &form_fields,
                &PathBuf::from(&output),
            )?;
            eprintln!("Filled: {output}");
        }
        Commands::Extract {
            input,
            output,
            assets: _,
            pretty,
        } => {
            let pdf_spec = extract::extract_pdf(&PathBuf::from(&input))?;

            let json = if pretty {
                serde_json::to_string_pretty(&pdf_spec)?
            } else {
                serde_json::to_string(&pdf_spec)?
            };

            if output == "-" {
                io::stdout().write_all(json.as_bytes())?;
                io::stdout().write_all(b"\n")?;
            } else {
                std::fs::write(&output, json.as_bytes())
                    .with_context(|| format!("failed to write {output}"))?;
                eprintln!("Extracted: {output}");
            }
        }
        Commands::Validate { spec } => {
            let json = read_input(&spec)?;
            let _pdf_spec: spec::PdfSpec =
                serde_json::from_str(&json).context("failed to parse JSON spec")?;
            eprintln!("Validation passed.");
        }
    }

    Ok(())
}

fn read_input(path: &str) -> Result<String> {
    if path == "-" {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        Ok(buf)
    } else {
        std::fs::read_to_string(path).with_context(|| format!("failed to read {path}"))
    }
}
