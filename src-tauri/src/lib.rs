use std::sync::Mutex;

struct AppState {
    counter: Mutex<i64>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn increment_counter(state: tauri::State<AppState>) -> i64 {
    let mut counter = state.counter.lock().unwrap();
    *counter += 1;
    *counter
}

#[tauri::command]
fn decrement_counter(state: tauri::State<AppState>) -> i64 {
    let mut counter = state.counter.lock().unwrap();
    *counter -= 1;
    *counter
}

#[tauri::command]
fn get_counter(state: tauri::State<AppState>) -> i64 {
    let counter = state.counter.lock().unwrap();
    *counter
}

#[tauri::command]
fn get_system_info() -> String {
    let info = format!(
        "OS: {}\nArch: {}\nHostname: {}\nPID: {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        hostname(),
        std::process::id()
    );
    info
}

fn hostname() -> String {
    std::process::Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            counter: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            increment_counter,
            decrement_counter,
            get_counter,
            get_system_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
