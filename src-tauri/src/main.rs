use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, WindowEvent,
};

const CHATGPT_URL: &str = "https://chatgpt.com/";

fn show_chatgpt(window: &tauri::Window) {
    if let Err(error) = window.show() {
        eprintln!("Failed to show window: {error}");
    }

    if let Err(error) = window.set_focus() {
        eprintln!("Failed to focus window: {error}");
    }

    if let Err(error) = window.eval(&format!("window.location.href = '{CHATGPT_URL}';")) {
        eprintln!("Failed to navigate window: {error}");
    }
}

fn main() {
    let open_chat = CustomMenuItem::new("open_chat".to_string(), "Open ChatGPT");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(open_chat).add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                if let Some(window) = app.get_window("main") {
                    show_chatgpt(&window);
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open_chat" => {
                    if let Some(window) = app.get_window("main") {
                        show_chatgpt(&window);
                    }
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().ok();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
