use clap::Parser;
use is_terminal::IsTerminal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod model_registry;
mod tokenizer;

use model_registry::ModelRegistry;
use tokenizer::count_tokens;

/// Detect programming language from file extension
fn detect_language(ext: &str) -> &'static str {
    match ext {
        "rs" => "Rust",
        "py" => "Python",
        "js" => "JavaScript",
        "jsx" => "JavaScript (JSX)",
        "ts" => "TypeScript",
        "tsx" => "TypeScript (TSX)",
        "md" => "Markdown",
        "toml" => "TOML",
        "yaml" | "yml" => "YAML",
        "json" => "JSON",
        "go" => "Go",
        "java" => "Java",
        "cpp" | "cc" | "cxx" => "C++",
        "c" => "C",
        "h" | "hpp" => "C/C++ Header",
        "sh" | "bash" => "Shell",
        "rb" => "Ruby",
        "php" => "PHP",
        "swift" => "Swift",
        "kt" | "kts" => "Kotlin",
        "scala" => "Scala",
        "sql" => "SQL",
        "html" | "htm" => "HTML",
        "css" => "CSS",
        "scss" | "sass" => "SCSS/SASS",
        "vue" => "Vue",
        "svelte" => "Svelte",
        "zig" => "Zig",
        "nim" => "Nim",
        "ex" | "exs" => "Elixir",
        "erl" | "hrl" => "Erlang",
        "hs" => "Haskell",
        "lua" => "Lua",
        "r" | "R" => "R",
        "dart" => "Dart",
        "xml" => "XML",
        "proto" => "Protobuf",
        "dockerfile" => "Dockerfile",
        "makefile" => "Makefile",
        _ => "Other",
    }
}

/// Map filename (no extension) to language
fn detect_language_from_filename(name: &str) -> &'static str {
    match name.to_lowercase().as_str() {
        "makefile" | "gnumakefile" => "Makefile",
        "dockerfile" => "Dockerfile",
        "cargo.lock" | "poetry.lock" | "package-lock.json" | "yarn.lock" => "Lock File",
        "license" | "licence" | "license.md" | "licence.md" => "License",
        "readme" | "readme.md" | "readme.rst" => "Documentation",
        "changelog" | "changelog.md" => "Changelog",
        ".gitignore" => "Git Config",
        ".dockerignore" => "Docker Config",
        "justfile" => "Justfile",
        "procfile" => "Procfile",
        _ => "Other",
    }
}

/// Supported text file extensions
const SUPPORTED_EXTENSIONS: &[&str] = &[
    "rs",
    "py",
    "js",
    "jsx",
    "ts",
    "tsx",
    "md",
    "toml",
    "yaml",
    "yml",
    "json",
    "go",
    "java",
    "cpp",
    "cc",
    "cxx",
    "c",
    "h",
    "hpp",
    "sh",
    "bash",
    "rb",
    "php",
    "swift",
    "kt",
    "kts",
    "scala",
    "sql",
    "html",
    "htm",
    "css",
    "scss",
    "sass",
    "vue",
    "svelte",
    "zig",
    "nim",
    "ex",
    "exs",
    "erl",
    "hrl",
    "hs",
    "lua",
    "r",
    "dart",
    "xml",
    "proto",
    "txt",
    "csv",
    "tsv",
    "env",
    "cfg",
    "conf",
    "ini",
    "lock",
    "dockerfile",
    "makefile",
];

#[derive(Parser)]
#[command(
    name = "ctx-budget",
    version,
    about = "Analyze token distribution across your codebase",
    long_about = "ctx-budget scans your project and counts tokens per file, helping you plan\nwhich files to include in LLM context windows.\n\nSupports 30+ languages, handles mixed encodings, excludes common build/dependency\ndirectories automatically, and outputs in text, JSON, or CSV formats.",
    author = "ctx-budget maintainers",
    after_help = "Examples:\n  ctx-budget .                        # Scan current directory\n  ctx-budget . --model gpt-6-astra    # Use GPT-6 Astra's 1.05M window\n  ctx-budget . --output json | jq .   # JSON for automation\n  ctx-budget . --limit 10             # Top 10 biggest files"
)]
struct Args {
    /// Path to scan (default: current directory)
    #[arg(default_value = ".")]
    path: String,

    /// LLM model name (loads context window from models.toml)
    #[arg(long, default_value = "gpt-6-astra")]
    model: String,

    /// Max number of files to show in output
    #[arg(long, default_value = "20")]
    limit: usize,

    /// Directories to skip (comma-separated)
    #[arg(
        long,
        value_delimiter = ',',
        default_value = "node_modules,.git,target,__pycache__,.venv,vendor,.pytest_cache,dist,build,.next,.nuxt,coverage,.cache"
    )]
    exclude_dirs: Vec<String>,

    /// Output format: text, json, csv, or auto (terminal→text, piped→json)
    #[arg(long, default_value = "auto")]
    output: OutputFormat,

    /// Show all files (not just top N)
    #[arg(long)]
    all: bool,

    /// Include hidden files (those starting with '.')
    #[arg(long)]
    hidden: bool,

    /// Path to models.toml (auto-detect in current dir or crate root if not provided)
    #[arg(long)]
    models_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum OutputFormat {
    Auto,
    Text,
    Json,
    Csv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Auto => write!(f, "auto"),
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
        }
    }
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "auto" => Ok(OutputFormat::Auto),
            "text" | "txt" => Ok(OutputFormat::Text),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(format!(
                "unknown format: '{s}'. Valid: auto, text, json, csv"
            )),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FileReport {
    path: String,
    tokens: usize,
    chars: usize,
    language: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Report {
    model: String,
    model_context_window: usize,
    scanned_at: String,
    summary: SummaryStat,
    files: Vec<FileReport>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SummaryStat {
    files_scanned: usize,
    total_chars: usize,
    total_tokens: usize,
    languages: HashMap<String, usize>,
}

/// Check if a path component should cause exclusion
fn should_exclude(path: &Path, exclude_dirs: &HashSet<String>) -> bool {
    path.iter().any(|component| {
        if let Some(name) = component.to_str() {
            exclude_dirs.contains(name)
        } else {
            false
        }
    })
}

/// Try to locate models.toml (in current dir, then project root)
fn locate_models_toml() -> Option<std::path::PathBuf> {
    // Try current directory
    let current = Path::new("models.toml");
    if current.exists() {
        return Some(current.to_path_buf());
    }

    // Try project root (parent of Cargo.toml)
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let root = Path::new(&manifest_dir).join("models.toml");
        if root.exists() {
            return Some(root);
        }
    }

    None
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: path '{}' does not exist", args.path);
        std::process::exit(1);
    }

    // Load model registry: external file wins, otherwise use embedded database
    let registry = if let Some(p) = args.models_path.as_deref() {
        let ext_path = Path::new(p);
        ModelRegistry::from_toml(ext_path).unwrap_or_else(|e| {
            eprintln!(
                "Warning: Failed to load models from '{}': {}. Falling back to embedded database.",
                p, e
            );
            ModelRegistry::load_embedded().expect("embedded models.toml is invalid")
        })
    } else if let Some(found) = locate_models_toml() {
        ModelRegistry::from_toml(&found).unwrap_or_else(|e| {
            eprintln!(
                "Warning: Failed to load '{}': {}. Falling back to embedded database.",
                found.display(),
                e
            );
            ModelRegistry::load_embedded().expect("embedded models.toml is invalid")
        })
    } else {
        ModelRegistry::load_embedded().expect("embedded models.toml is invalid")
    };

    // Resolve model
    let model_spec = registry.get(&args.model).cloned().ok_or_else(|| {
        anyhow::anyhow!(
            "Unknown model '{}'. Available models: {}",
            args.model,
            registry
                .list_models()
                .iter()
                .map(|(id, _)| id.as_str())
                .take(5)
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;

    let exclude_set: HashSet<String> = args.exclude_dirs.iter().cloned().collect();
    let mut total_files = 0;
    let mut total_chars = 0;
    let mut total_tokens = 0;
    let mut lang_counts: HashMap<String, usize> = HashMap::new();
    let mut per_file: Vec<FileReport> = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();

        // Skip excluded directories
        if should_exclude(file_path, &exclude_set) {
            continue;
        }

        // Skip hidden files unless --hidden
        if !args.hidden {
            let name = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.starts_with('.') && name.len() > 1 {
                continue;
            }
        }

        // Skip symlinks (prevent infinite loops)
        if file_path.is_symlink() {
            continue;
        }

        // Determine file extension or filename
        let ext = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let filename = file_path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        let supported = if ext.is_empty() {
            !detect_language_from_filename(filename).eq("Other")
        } else {
            SUPPORTED_EXTENSIONS.contains(&ext.as_str())
        };

        if !supported {
            continue;
        }

        let language = if ext.is_empty() {
            detect_language_from_filename(filename).to_string()
        } else {
            detect_language(&ext).to_string()
        };

        match fs::read_to_string(file_path) {
            Ok(content) => {
                let tokens = count_tokens(&content);
                let chars = content.len();

                total_files += 1;
                total_chars += chars;
                total_tokens += tokens;

                lang_counts
                    .entry(language.clone())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);

                per_file.push(FileReport {
                    path: file_path
                        .strip_prefix(path)
                        .unwrap_or(file_path)
                        .to_string_lossy()
                        .into_owned(),
                    tokens,
                    chars,
                    language,
                });
            },
            Err(_) => {
                // Skip binary or unreadable files
                continue;
            },
        }
    }

    // Sort by tokens (descending)
    per_file.sort_by(|a, b| b.tokens.cmp(&a.tokens));
    let display_files = if args.all {
        per_file
    } else {
        per_file.into_iter().take(args.limit).collect()
    };

    let report = Report {
        model: format!(
            "{} ({} tokens context)",
            model_spec.name, model_spec.context_window
        ),
        model_context_window: model_spec.context_window,
        scanned_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        summary: SummaryStat {
            files_scanned: total_files,
            total_chars,
            total_tokens,
            languages: lang_counts,
        },
        files: display_files,
    };

    let output_fmt = match args.output {
        OutputFormat::Auto => {
            if std::io::stdout().is_terminal() {
                OutputFormat::Text
            } else {
                OutputFormat::Json
            }
        },
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
    println!();
    println!("Summary:");
    println!("  Files scanned: {}", report.summary.files_scanned);
    println!(
        "  Total chars:   {}",
        format_number(report.summary.total_chars)
    );
    println!(
        "  Total tokens:  {}",
        format_number(report.summary.total_tokens)
    );
    println!(
        "  % of {} context: {:.2}%",
        format_number(report.model_context_window),
        (report.summary.total_tokens as f64 / report.model_context_window as f64) * 100.0
    );
    println!();
    println!("Languages found: {}", report.summary.languages.len());
    let mut langs: Vec<_> = report.summary.languages.iter().collect();
    langs.sort_by(|a, b| b.1.cmp(a.1));
    for (lang, count) in langs.iter().take(10) {
        println!("  {:<20} {} file(s)", lang, count);
    }
    println!();
    println!(
        "Top {} files by token count:",
        limit.min(report.files.len())
    );
    for (i, file) in report.files.iter().enumerate() {
        println!(
            "  {:>3}. {:>7} tokens | {:<20} | {}",
            i + 1,
            format_number(file.tokens),
            file.language,
            file.path
        );
    }
}

fn print_csv(report: &Report) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    wtr.write_record(["path", "tokens", "chars", "language"])?;
    for file in &report.files {
        wtr.write_record([
            &file.path,
            &file.tokens.to_string(),
            &file.chars.to_string(),
            &file.language,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Format number with commas: 1234567 -> "1,234,567"
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            result.push(',');
        }
        result.push(c);
    }
    result.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_exclude_node_modules() {
        let exclude = ["node_modules"].iter().map(|s| s.to_string()).collect();
        assert!(should_exclude(
            Path::new("node_modules/pkg/file.js"),
            &exclude
        ));
    }

    #[test]
    fn should_not_exclude_normal_path() {
        let exclude = ["node_modules"].iter().map(|s| s.to_string()).collect();
        assert!(!should_exclude(Path::new("src/main.rs"), &exclude));
    }

    #[test]
    fn output_format_parsing() {
        assert!(matches!(
            "auto".parse::<OutputFormat>(),
            Ok(OutputFormat::Auto)
        ));
        assert!(matches!(
            "json".parse::<OutputFormat>(),
            Ok(OutputFormat::Json)
        ));
        assert!(matches!(
            "csv".parse::<OutputFormat>(),
            Ok(OutputFormat::Csv)
        ));
        assert!(matches!(
            "text".parse::<OutputFormat>(),
            Ok(OutputFormat::Text)
        ));
        assert!(matches!(
            "txt".parse::<OutputFormat>(),
            Ok(OutputFormat::Text)
        ));
        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn format_number_test() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(42), "42");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(1234567), "1,234,567");
        assert_eq!(format_number(1234567890), "1,234,567,890");
    }

    #[test]
    fn detect_language_works() {
        assert_eq!(detect_language("rs"), "Rust");
        assert_eq!(detect_language("py"), "Python");
        assert_eq!(detect_language("js"), "JavaScript");
        assert_eq!(detect_language("ts"), "TypeScript");
        assert_eq!(detect_language("go"), "Go");
        assert_eq!(detect_language("unknown"), "Other");
    }

    #[test]
    fn detect_language_from_filename_works() {
        assert_eq!(detect_language_from_filename("Makefile"), "Makefile");
        assert_eq!(detect_language_from_filename("Dockerfile"), "Dockerfile");
        assert_eq!(detect_language_from_filename("Cargo.lock"), "Lock File");
        assert_eq!(detect_language_from_filename("unknown.xyz"), "Other");
    }
}
