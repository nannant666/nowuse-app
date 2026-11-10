#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

/// 每次页面加载前注入：把 window.open 与 target=_blank 收敛成当前窗口跳转。
/// 桌面壳没有新窗口可用，否则站内的公告、协议这类链接点了没反应。
const INIT_SCRIPT: &str = r#"
(function () {
  try {
    window.open = function (url) {
      if (typeof url === 'string' && url.length > 0) {
        window.location.href = url;
      }
      return null;
    };
    document.addEventListener('click', function (e) {
      var node = e.target;
      var link = node && node.closest ? node.closest('a') : null;
      if (link && link.href && link.target === '_blank') {
        e.preventDefault();
        window.location.href = link.href;
      }
    }, true);
  } catch (err) {}
})();
"#;

/// 在系统默认浏览器里打开安装包下载地址。
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
        .setup(|app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Nowuse AI")
                .inner_size(1280.0, 820.0)
                .min_inner_size(960.0, 640.0)
                .center()
                .initialization_script(INIT_SCRIPT)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
