// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ai;
mod commands;
mod db;
mod scheduler;

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
        ])
        .setup(|app| {
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

            // Initialize AI client state
            app.manage(commands::ai::AiClientState::new());

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
