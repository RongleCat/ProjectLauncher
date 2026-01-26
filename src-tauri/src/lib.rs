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

#[cfg(target_os = "macos")]
use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // macOS: 启动时设置为 Accessory 模式，不显示在 Dock
            #[cfg(target_os = "macos")]
            unsafe {
                let ns_app = NSApp();
                ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
            }

            // 初始化应用状态
            let app_data_dir = app.path().app_data_dir()?;

            // 确保数据目录存在
            fs::create_dir_all(&app_data_dir)?;

            let cache_manager = CacheManager::new(app_data_dir.clone());

            // 加载配置
            let config_path = app_data_dir.join("config.json");
            println!("配置文件路径: {:?}", config_path);
            let config = if config_path.exists() {
                match fs::read_to_string(&config_path) {
                    Ok(content) => {
                        match serde_json::from_str::<Config>(&content) {
                            Ok(cfg) => {
                                println!("配置加载成功，启动器数量: {}", cfg.launchers.len());
                                cfg
                            }
                            Err(e) => {
                                eprintln!("配置文件解析失败: {}, 使用默认配置", e);
                                Config::default()
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("读取配置文件失败: {}, 使用默认配置", e);
                        Config::default()
                    }
                }
            } else {
                println!("配置文件不存在，使用默认配置");
                Config::default()
            };

            app.manage(AppState {
                cache_manager: Mutex::new(cache_manager),
                config: Mutex::new(config.clone()),
            });

            // 注册启动器快捷键（使用 tauri-plugin-global-shortcut）
            let shortcut_manager = ShortcutManager::new(app.handle().clone());
            for launcher in &config.launchers {
                if let Some(ref shortcut) = launcher.shortcut {
                    if let Err(e) = shortcut_manager.register_for_launcher(shortcut, &launcher.id) {
                        eprintln!("注册启动器快捷键失败 [{}]: {}", launcher.name, e);
                    } else {
                        println!("启动器快捷键已注册: {} -> {}", launcher.name, shortcut);
                    }
                }
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

            // Build tray icon with platform-specific settings
            let mut tray_builder = TrayIconBuilder::new();

            #[cfg(target_os = "macos")]
            {
                // macOS: Use template icon for system theme adaptation (light/dark mode)
                let tray_icon_path = app.path().resolve("icons/tray-icon.png", tauri::path::BaseDirectory::Resource)?;
                tray_builder = tray_builder
                    .icon(tauri::image::Image::from_path(tray_icon_path)?)
                    .icon_as_template(true);
            }

            #[cfg(not(target_os = "macos"))]
            {
                // Windows/Linux: Use default app icon
                tray_builder = tray_builder.icon(app.default_window_icon().unwrap().clone());
            }

            let _ = tray_builder
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
            commands::project::update_project_launcher,
            commands::project::update_project_top,
            commands::project::add_custom_project,
            commands::project::remove_custom_project,
            // 启动器相关
            commands::launcher::launch_project,
            commands::launcher::get_launchers,
            commands::launcher::add_launcher,
            commands::launcher::update_launcher,
            commands::launcher::remove_launcher,
            // 快捷键相关
            commands::shortcut::check_shortcut_conflict,
            commands::shortcut::register_launcher_shortcut,
            commands::shortcut::unregister_launcher_shortcut,
            // 窗口相关
            commands::window::show_search_window,
            commands::window::hide_search_window,
            commands::window::show_settings_window,
            commands::window::hide_settings_window,
            commands::window::quit_app,
            // 配置相关
            commands::config::get_config,
            commands::config::save_config,
            commands::config::set_autostart,
            commands::config::get_autostart_status,
            // 导入导出相关
            commands::export::export_settings,
            commands::export::import_settings,
            commands::export::preview_import,
        ])
        // 全局窗口事件处理：所有窗口关闭时隐藏而非关闭
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                // 隐藏窗口而不是关闭，保持应用在托盘运行
                let _ = window.hide();
                api.prevent_close();
                println!("窗口 [{}] 关闭请求已拦截，已隐藏", window.label());

                // macOS: 设置窗口关闭时切换回 Accessory 模式
                #[cfg(target_os = "macos")]
                if window.label() == "settings" {
                    unsafe {
                        let ns_app = NSApp();
                        ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                    }
                }
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, event| {
            match event {
                // 始终阻止应用退出，只有托盘菜单的 std::process::exit(0) 才能真正退出
                // 这确保了窗口关闭、Cmd+Q 等操作都不会导致应用退出
                tauri::RunEvent::ExitRequested { api, .. } => {
                    api.prevent_exit();
                    println!("退出请求已阻止，应用继续在托盘运行");
                }
                _ => {}
            }
        });
}
