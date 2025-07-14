mod gitproject;
mod config;
mod ollama_client;
mod prompt_builder;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "A tool to process files with a given prompt")]
struct Args {
    #[arg(help = "The name of the file to process")]
    filename: String,
    #[arg(help = "The prompt to use for processing the file")]
    prompt: String,
    #[arg(long, help = "Enable dry run mode, no changes will be made")]
    dry_run: bool,
    #[arg(short, long, help = "Automatic confirmation, skip all prompts")]
    yes: bool,
    #[arg(long, help = "Optional configuration file path")]
    config: Option<String>,
}

fn main() {
    if !gitproject::is_git_repository() {
        eprintln!("This tool must be run inside a git repository.");
        std::process::exit(1);
    }
    let args = Args::parse();
    println!("File Name: {}", args.filename);
    println!("Prompt: {}", args.prompt);
    if args.dry_run {
        println!("Dry run mode is enabled. No changes will be made.");
    }
    if args.yes {
        println!("Automatic confirmation is enabled. All prompts will be skipped.");
    }
    let config_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Could not determine home directory.");
        std::process::exit(1);
    }).join(".config");
    if !config_dir.exists() && let Err(e) = std::fs::create_dir_all(&config_dir) {
        eprintln!("Error creating configuration directory: {}", e);
        std::process::exit(1);
    }
    let default_config_path = config_dir.join("code-assistant.json");
    let config_path = args.config.as_deref().unwrap_or(default_config_path.to_str().unwrap());
    let config = config::Config::load(config_path).unwrap_or_else(|e| {
        eprintln!("Error loading configuration: {}", e);
        std::process::exit(1);
    });
    let prompt = prompt_builder::build_prompt(&args.filename, &args.prompt).unwrap_or_else(|e| {
        eprintln!("Error building prompt: {}", e);
        std::process::exit(1);
    });
    let response = ollama_client::generate(config, &prompt).unwrap_or_else(|e| {
        eprintln!("Error generating response: {}", e);
        std::process::exit(1);
    });
    termimad::print_inline(&response);
}
