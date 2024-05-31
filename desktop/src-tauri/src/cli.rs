use clap::Parser;
use std::path::{Path, PathBuf};
use std::{env, process};
use tauri::App;
use vibe::config::{get_models_folder, TranscribeOptions};
use vibe::model;

pub fn is_cli_detected() -> bool {
    // Get the command-line arguments as an iterator
    let args: Vec<String> = env::args().collect();

    // Check if any argument starts with "--"
    for arg in &args {
        if arg.starts_with("--") {
            return true;
        }
    }
    false
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to model
    #[arg(long)]
    model: PathBuf,

    /// Path to file to transcribe
    #[arg(long)]
    file: PathBuf,

    /// Language to transcribe (default: "en")
    #[arg(long, default_value = "en")]
    language: Option<String>,

    /// Temperature (default: 0.4)
    #[arg(long, default_value = "0.4")]
    temperature: Option<f32>,

    /// Number of threads (default: 4)
    #[arg(long, default_value = "4")]
    n_threads: Option<i32>,

    /// Whether to translate (default: false)
    #[arg(long)]
    translate: Option<bool>,

    /// Initial prompt (default: None)
    #[arg(long)]
    init_prompt: Option<String>,
}

fn prepare_model_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    // Check if relative to current dir
    if path.exists() {
        return path.to_path_buf();
    }
    // Check if relative to app config exists
    let relative_to_models_folder = get_models_folder().unwrap().join(path);
    if relative_to_models_folder.exists() {
        return relative_to_models_folder;
    }
    path.to_path_buf()
}

pub fn run(app: &App) {
    #[cfg(target_os = "macos")]
    crate::dock::set_dock_visible(false);

    let args = Args::parse();
    let mut options = TranscribeOptions {
        path: args.file,
        model_path: args.model,
        lang: args.language,
        init_prompt: args.init_prompt,
        n_threads: args.n_threads,
        temperature: args.temperature,
        translate: args.translate,
        verbose: false,
    };
    options.model_path = prepare_model_path(&options.model_path);
    let transcript = model::transcribe(&options, None, None, None).unwrap();
    println!("{}", transcript.as_srt());
    app.cleanup_before_exit();
    process::exit(0);
}
