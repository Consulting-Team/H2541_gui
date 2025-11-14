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

    let (mut rx, mut child) = sidecar_command.spawn().expect("Failed to spawn side car.");

    // tauri::async_runtime::spawn(async move {
    //     while let Some(event) = rx.recv().await {
    //         if let CommandEvent::Stdout(line_bytes) = event {
    //             let line = String::from_utf8_lossy(&line_bytes);
    //             let line = line.trim();
    //             window.emit("message", Some(format!("'{}'", line))).expect("failed to emit event");
    //             child.write("message from Rust\n".as_bytes()).unwrap();

    //             // println!("WebviewWindow: {}", window.label());
    //             // println!("{}", line.to_string());
    //         }
    //     }
    // });

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line_bytes) = event {
            let line = String::from_utf8_lossy(&line_bytes);
            let line = line.trim();

            println!("{line}");
            window
                .emit("message", Some(format!("'{}'", line)))
                .expect("Failed to emit event.");
            // child.write("message from Rust\n".as_bytes()).unwrap();
            // child.write("message from Rust\n".as_bytes()).map_err(|e| e.to_string())?;
            child
                .write("message from Rust\n".as_bytes())
                .expect("Failed to write message to child.");
        }
        // else if let CommandEvent::Error(err) = event {
        //     Err(err)?
        // } else {
        //     continue;
        // }
    }

    app.emit("onProgress", false).expect("failed to emit event.");
    Ok("done".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, export_report])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
