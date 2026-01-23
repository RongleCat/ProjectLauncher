mod commands;
mod models;
mod services;

use tauri::{Emitter, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState, TrayIconEvent};
use commands::project::AppState;
use services::cache_manager::CacheManager;
use services::shortcut_manager::ShortcutManager;
use models::config::Config;
use std::sync::Mutex;
use std::fs;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // 初始化应用状态
            let app_data_dir = app.path().app_data_dir()?;

            // 确保数据目录存在
            fs::create_dir_all(&app_data_dir)?;

            let cache_manager = CacheManager::new(app_data_dir.clone());

            // 加载配置
            let config_path = app_data_dir.join("config.json");
            let config = if config_path.exists() {
                let content = fs::read_to_string(config_path)?;
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                Config::default()
            };

            app.manage(AppState {
                cache_manager: Mutex::new(cache_manager),
                config: Mutex::new(config.clone()),
            });

            // 注册全局快捷键
            if let Some(ref shortcut) = config.global_shortcut {
                let manager = ShortcutManager::new(app.handle().clone());
                if let Err(e) = manager.register(shortcut) {
                    eprintln!("注册全局快捷键失败: {}", e);
                } else {
                    println!("全局快捷键已注册: {}", shortcut);
                }
            }

            // 注册启动器快捷键
            for launcher in &config.launchers {
                if let Some(ref shortcut) = launcher.shortcut {
                    let manager = ShortcutManager::new(app.handle().clone());
                    if let Err(e) = manager.register_for_launcher(shortcut, &launcher.id) {
                        eprintln!("注册启动器快捷键失败 [{}]: {}", launcher.name, e);
                    } else {
                        println!("启动器快捷键已注册: {} -> {}", launcher.name, shortcut);
                    }
                }
            }

            // 窗口关闭事件处理（隐藏到托盘）
            if let Some(window) = app.get_webview_window("search") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // 隐藏窗口而不是关闭
                        let _ = window_clone.hide();
                        api.prevent_close();
                    }
                });
            }

            // 开发工具（需要时可以手动打开：右键 -> Inspect Element）
            // #[cfg(debug_assertions)]
            // {
            //     if let Some(window) = app.get_webview_window("search") {
            //         window.open_devtools();
            //     }
            // }

            // 创建托盘图标和菜单
            let menu = MenuBuilder::new(app)
                .item(&MenuItemBuilder::with_id("show", "显示搜索框").build(app)?)
                .item(&MenuItemBuilder::with_id("settings", "打开设置").build(app)?)
                .item(&MenuItemBuilder::with_id("refresh", "刷新项目缓存").build(app)?)
                .separator()
                .item(&MenuItemBuilder::with_id("quit", "退出").build(app)?)
                .build()?;

            let _ = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = commands::window::show_search_window(app_clone).await;
                            });
                        }
                        "settings" => {
                            let app_clone = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = commands::window::show_settings_window(app_clone).await;
                            });
                        }
                        "refresh" => {
                            let _ = app.emit("refresh-projects", ());
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button, button_state, .. } = event {
                        if button == MouseButton::Left && button_state == MouseButtonState::Up {
                            let app = tray.app_handle().clone();
                            tauri::async_runtime::spawn(async move {
                                let _ = commands::window::show_search_window(app).await;
                            });
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 项目相关
            commands::project::get_cached_projects,
            commands::project::force_rescan,
            commands::project::detect_project_type,
            commands::project::batch_detect_types,
            commands::project::increment_project_hits,
            // 启动器相关
            commands::launcher::launch_project,
            commands::launcher::get_launchers,
            commands::launcher::add_launcher,
            commands::launcher::update_launcher,
            commands::launcher::remove_launcher,
            // 快捷键相关
            commands::shortcut::register_global_shortcut,
            commands::shortcut::unregister_global_shortcut,
            commands::shortcut::check_shortcut_conflict,
            commands::shortcut::register_launcher_shortcut,
            commands::shortcut::unregister_launcher_shortcut,
            // 窗口相关
            commands::window::show_search_window,
            commands::window::hide_search_window,
            commands::window::show_settings_window,
            commands::window::quit_app,
            // 配置相关
            commands::config::get_config,
            commands::config::save_config,
            commands::config::set_autostart,
            commands::config::get_autostart_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
