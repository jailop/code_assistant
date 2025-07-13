mod gitproject;
mod editor;
mod config;
mod ollama_client;

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
    let content = editor::read_file(&args.filename);
    match content {
        Ok(data) => {
            println!("File content read successfully.");
            println!("Content length: {}", data.len());
        }
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            std::process::exit(1);
        }
    }
}
