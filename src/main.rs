use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
#[command(name = "ctx-budget")]
#[command(about = "Analyze token budget per file in a repository")]
struct Args {
    #[arg(default_value = ".")]
    path: String,

    #[arg(long, default_value = "gpt-4o")]
    model: String,

    #[arg(long, default_value = "20")]
    limit: usize,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Error: path {} does not exist", args.path);
        std::process::exit(1);
    }

    println!("=== ctx-budget Report ===");
    println!("Model: {}", args.model);
    println!("Path:  {}", path.display());

    let mut total_files = 0;
    let mut total_chars = 0;
    let mut per_file: Vec<(String, usize)> = Vec::new();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        
        if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if matches!(ext_str.as_ref(), "rs" | "py" | "js" | "ts" | "md" | "toml" | "yaml" | "json") {
                if let Ok(content) = fs::read_to_string(path) {
                    let chars = content.len();
                    let tokens_approx = (chars as f32 / 4.0) as usize;
                    
                    total_files += 1;
                    total_chars += chars;
                    
                    let rel_path = path
                        .strip_prefix(Path::new(&args.path))
                        .map(|p| p.to_string_lossy().to_string())
                        .unwrap_or_else(|_| path.to_string_lossy().to_string());
                    
                    per_file.push((rel_path, tokens_approx));
                }
            }
        }
    }

    per_file.sort_by(|a, b| b.1.cmp(&a.1));

    println!("\nSummary:");
    println!("  Files scanned: {}", total_files);
    println!("  Total chars: {}", total_chars);
    println!("  Est. tokens: ~{}", (total_chars as f32 / 4.0) as usize);

    println!("\nTop {} files by token count:", args.limit);
    for (i, (f, t)) in per_file.iter().take(args.limit).enumerate() {
        println!("  {:2}. {:>6} tokens | {}", i + 1, t, f);
    }

    Ok(())
}
