// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod db;
mod scheduler;
mod summary;  // ✨ Phase 5: Summary module
mod window;   // macOS window customization

use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // Load environment variables from .env file
    #[cfg(debug_assertions)]
    {
        dotenv::dotenv().ok();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::task::create_task,
            commands::task::get_task,
            commands::task::update_task,
            commands::task::delete_task,
            commands::task::list_tasks,
            commands::task::get_db_version,
            commands::ai::parse_text_input,
            commands::ai::parse_image_input,
            commands::ai::parse_image_for_operations,
            commands::ai::ai_health_check,
            commands::ai::get_ai_provider,
            commands::test::test_kimi_connection,
            commands::notification::send_notification,
            commands::notification::check_expiring_tasks,
            commands::notification::test_notification,
            // ✨ Phase 5: Summary commands
            commands::summary::generate_summary,
            commands::summary::get_or_generate_summary,
            commands::summary::list_summaries,
            commands::summary::get_summary,
            commands::summary::delete_summary,
            commands::summary::export_summary,
            // ✨ Phase 5.3: Settings commands
            commands::settings::get_auto_summary_settings,
            commands::settings::update_auto_summary_settings,
            commands::settings::get_notification_settings,
            commands::settings::update_notification_settings,
        ])
        .setup(|app| {
            // ✨ 设置 macOS 交通灯位置
            if let Some(main_window) = app.get_webview_window("main") {
                window::setup_traffic_light_position(&main_window);
            }

            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let db_path = app_data_dir.join("intento.db");
            let database = db::Database::new(db_path).expect("Failed to initialize database");

            // Database is Clone, so we can manage it directly
            app.manage(database.clone());

            // Initialize AI client state (shared between ai and summary commands)
            app.manage(commands::ai::AiClientState::new());

            // Initialize summary AI client state
            app.manage(commands::summary::AiClientState::new());

            // Initialize and start the task scheduler
            let app_handle = app.handle().clone();
            let db_for_scheduler = database.clone();

            tauri::async_runtime::spawn(async move {
                match scheduler::TaskScheduler::new(app_handle, db_for_scheduler).await {
                    Ok(task_scheduler) => {
                        // Add scheduled jobs
                        if let Err(e) = task_scheduler.add_deadline_reminder_job().await {
                            eprintln!("Failed to add deadline reminder job: {}", e);
                        }

                        if let Err(e) = task_scheduler.add_daily_summary_job().await {
                            eprintln!("Failed to add daily summary job: {}", e);
                        }

                        // Add automatic summary generation jobs (Phase 5.2)
                        if let Err(e) = task_scheduler.add_auto_summary_jobs().await {
                            eprintln!("Failed to add auto summary jobs: {}", e);
                        }

                        // Start the scheduler
                        if let Err(e) = task_scheduler.start().await {
                            eprintln!("Failed to start scheduler: {}", e);
                        } else {
                            println!("Task scheduler started successfully");
                        }

                        // Keep scheduler alive by managing it in the app state
                        // Note: We don't drop the scheduler, it stays alive for the app lifetime
                    }
                    Err(e) => {
                        eprintln!("Failed to initialize task scheduler: {}", e);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
