// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod cmd;
mod config;
mod panic_hook;
mod setup;
mod utils;

#[cfg(target_os = "macos")]
mod dock;

#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_os = "windows"))]
mod x86_features;

use tauri::Manager;
use tauri_plugin_window_state::StateFlags;

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

fn main() {
    // Attach console in Windows:
    #[cfg(windows)]
    cli::attach_console();

    env_logger::init();
    log::debug!("Vibe App Running");

    tauri::Builder::default()
        .setup(|app| setup::setup(app))
        .plugin(
            tauri_plugin_window_state::Builder::default()
                .with_state_flags(!StateFlags::VISIBLE)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit("single-instance", Payload { args: argv, cwd }).unwrap();
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd::transcribe,
            cmd::download_model,
            cmd::get_default_model_path,
            cmd::get_commit_hash,
            cmd::is_online,
            cmd::get_path_dst,
            cmd::open_path,
            cmd::get_x86_features,
            cmd::get_save_path,
            cmd::audio::get_audio_devices,
            cmd::audio::start_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
