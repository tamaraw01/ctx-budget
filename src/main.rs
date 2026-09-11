use std::fs;
use std::path::Path;
use std::collections::HashSet;
use walkdir::WalkDir;
use clap::Parser;
use serde::{Deserialize, Serialize};

mod tokenizer;
use tokenizer::count_tokens;

#[derive(Parser)]
#[command(name = "ctx-budget")]
#[command(about = "Analyze token budget per file in a repository")]
#[command(author = "Augie via Francois")]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(long, default_value = "gpt-4o")]
    model: String,

    #[arg(long, default_value = "20")]
    limit: usize,

    #[arg(long, value_delimiter = ',', default_value = "node_modules,.git,target,__pycache__,.venv,vendor,.pytest_cache")]
    exclude_dirs: Vec<String>,

    #[arg(long, default_value = "auto")]
    output: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Auto,
    Text,
    Json,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(OutputFormat::Auto),
            "text" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(format!("unknown format: {}", s)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileReport {
    path: String,
    tokens: usize,
    chars: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    model: String,
    scanned_at: String,
    summary: SummaryStat,
    files: Vec<FileReport>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SummaryStat {
    files_scanned: usize,
    total_chars: usize,
    total_tokens: usize,
}

fn should_exclude(path: &Path, exclude_dirs: &HashSet<String>) -> bool {
    path.iter().any(|component| {
        if let Some(name) = component.to_str() {
            exclude_dirs.contains(name)
        } else {
            false
        }
    })
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: path {} does not exist", args.path);
        std::process::exit(1);
    }

    let exclude_set: HashSet<String> = args.exclude_dirs.iter().cloned().collect();
    let mut total_files = 0;
    let mut total_chars = 0;
    let mut total_tokens = 0;
    let mut per_file: Vec<FileReport> = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();

        if should_exclude(file_path, &exclude_set) {
            continue;
        }

        if let Some(ext) = file_path.extension() {
            let ext_str = ext.to_string_lossy();
            if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts" | "md" | "toml" | "yaml" | "json" | "go" | "java" | "cpp" | "c" | "h" | "sh" | "bash") {
                if let Ok(content) = fs::read_to_string(file_path) {
                    let chars = content.len();
                    let tokens = count_tokens(&content);

                    total_files += 1;
                    total_chars += chars;
                    total_tokens += tokens;

                    let rel_path = file_path
                        .strip_prefix(path)
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| file_path.to_string_lossy().to_string());

                    per_file.push(FileReport {
                        path: rel_path,
                        tokens,
                        chars,
                    });
                }
            }
        }
    }

    per_file.sort_by(|a, b| b.tokens.cmp(&a.tokens));

    let report = Report {
        model: args.model.clone(),
        scanned_at: format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()),
        summary: SummaryStat {
            files_scanned: total_files,
            total_chars,
            total_tokens,
        },
        files: per_file.iter().take(args.limit).cloned().collect(),
    };

    let output_fmt = match args.output {
        OutputFormat::Auto => {
            if atty::is(atty::Stream::Stdout) {
                OutputFormat::Text
            } else {
                OutputFormat::Json
            }
        }
        other => other,
    };

    match output_fmt {
        OutputFormat::Text => print_text(&report, args.limit),
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Csv => print_csv(&report)?,
        OutputFormat::Auto => unreachable!(),
    }

    Ok(())
}

fn print_text(report: &Report, limit: usize) {
    println!("=== ctx-budget Report ===");
    println!("Model: {}", report.model);
    println!("Scanned: {}", report.scanned_at);
    println!();
    println!("Summary:");
    println!("  Files scanned: {}", report.summary.files_scanned);
    println!("  Total chars: {}", report.summary.total_chars);
    println!("  Total tokens: {}", report.summary.total_tokens);
    println!();
    println!("Top {} files by token count:", limit.min(report.files.len()));
    for (i, file) in report.files.iter().take(limit).enumerate() {
        println!("  {:2}. {:>6} tokens | {}", i + 1, file.tokens, file.path);
    }
}

fn print_csv(report: &Report) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(&["path", "tokens", "chars"])?;
    for file in &report.files {
        wtr.write_record(&[&file.path, &file.tokens.to_string(), &file.chars.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_exclude_node_modules() {
        let exclude = ["node_modules"].iter().map(|s| s.to_string()).collect();
        assert!(should_exclude(Path::new("node_modules/pkg/file.js"), &exclude));
    }

    #[test]
    fn should_not_exclude_normal_path() {
        let exclude = ["node_modules"].iter().map(|s| s.to_string()).collect();
        assert!(!should_exclude(Path::new("src/main.rs"), &exclude));
    }

    #[test]
    fn output_format_parsing() {
        assert!(matches!("auto".parse::<OutputFormat>(), Ok(OutputFormat::Auto)));
        assert!(matches!("json".parse::<OutputFormat>(), Ok(OutputFormat::Json)));
        assert!(matches!("csv".parse::<OutputFormat>(), Ok(OutputFormat::Csv)));
        assert!(matches!("text".parse::<OutputFormat>(), Ok(OutputFormat::Text)));
    }
}
