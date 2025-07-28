use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            // Inject CSS to hide DDG menu after the page loads
            window.eval(r#"
                const style = document.createElement('style');
                style.innerHTML = `
                    #aichat-side-menu-button {
                    display: none !important;
                    }
                `;
                document.head.appendChild(style);
            "#)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
