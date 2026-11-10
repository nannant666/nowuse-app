#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// 在系统默认浏览器里打开链接（安装包下载、外部页面）。
/// 启动页在 WebView 内调用 `window.__TAURI__.core.invoke('open_external', { url })`。
#[tauri::command]
fn open_external(url: String) -> Result<(), String> {
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("unsupported scheme".into());
    }
    opener::open(url).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_external])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
