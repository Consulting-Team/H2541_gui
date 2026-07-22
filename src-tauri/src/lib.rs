use regex::Regex;
use tauri::{AppHandle, Emitter, WebviewWindow};
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// async fn export_daily_report(app: tauri::AppHandle, window: tauri::Window, input: &str) -> Result<(), String> {

//     let sidecar_command = app
//         .shell()
//         .sidecar("cli")
//         .map_err(|e| e.to_string())?
//         .args(["-s", input, "-t", "voyage"]);

//     Ok(())
// }

#[tauri::command]
async fn get_average(
    app: AppHandle,
    window: WebviewWindow,
    input: &str,
    item: &str,
) -> Result<f64, String> {
    // app.emit("onProgress", true).expect("failed to emit event.");

    let sidecar_command = app
        .shell()
        .sidecar("average")
        .map_err(|e| e.to_string())?
        .args(["-s", input, "-i", item]);

    let (mut rx, mut child) = sidecar_command.spawn().expect("Failed to spawn side car.");
    let re = Regex::new(r#"\("([^"]*)"\)"#).unwrap();
    let mut value = 0.0;

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line_bytes) = &event {
            let line = String::from_utf8_lossy(line_bytes);
            let line = line.trim();

            // println!("{line}");

            let split = line.split(":").collect::<Vec<_>>();
            let str_value = split
                .get(1)
                .ok_or(format!("Failed to parse {} to object.", line))?
                .trim();
            value = str_value
                .parse::<f64>()
                .map_err(|e| format!("Failed to parse {str_value} to f64. {e}"))?;

            window
                .emit("message", Some(format!("'{}'", line)))
                .expect("Failed to emit event to message.");
            child
                .write("message from Rust\n".as_bytes())
                .expect("Failed to write message to child.");
        }

        if let CommandEvent::Stderr(tmp) = &event {
            let line = String::from_utf8_lossy(tmp);
            let line = line.trim();
            let msg = match re.captures(line) {
                Some(tokens) => tokens.get(1).map(|m| m.as_str()).expect(line),
                None => line,
            };

            // app.emit("onProgress", false)
            //     .expect("Failed to emit event to onProgress.");

            Err(msg)?
        }
    }

    // app.emit("onProgress", false)
    //     .expect("Failed to emit event to onProgress.");

    Ok(value)
}

#[tauri::command]
async fn export_report(
    app: AppHandle,
    window: WebviewWindow,
    input: &str,
    repo_type: &str,
) -> Result<String, String> {
    app.emit("onProgress", true).expect("failed to emit event.");

    let sidecar_command = app
        .shell()
        .sidecar("cli")
        .map_err(|e| e.to_string())?
        .args(["-s", input, "-t", repo_type]);
    // println!("{}", "-".repeat(50));

    let (mut rx, _) = sidecar_command.spawn().expect("Failed to spawn side car.");
    let re = Regex::new(r#"\("([^"]*)"\)"#).unwrap();

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Terminated(_) = &event {
            break;
        }

        if let CommandEvent::Stdout(line_bytes) = &event {
            let line = String::from_utf8_lossy(line_bytes);
            let line = line.trim();

            window
                .emit("message", Some(format!("'{}'", line)))
                .expect("Failed to emit event to message.");

            // child
            //     .write("message from Rust\n".as_bytes())
            //     .expect("Failed to write message to child.");
        }

        if let CommandEvent::Stderr(tmp) = &event {
            let line = String::from_utf8_lossy(tmp);
            let line = line.trim();
            let msg = match re.captures(line) {
                Some(tokens) => tokens.get(1).map(|m| m.as_str()).expect(line),
                None => line,
            };

            app.emit("onProgress", false)
                .expect("Failed to emit event to onProgress.");

            Err(msg)?
        }
    }

    app.emit("onProgress", false)
        .expect("Failed to emit event to onProgress.");

    Ok("done".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, export_report, get_average])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
