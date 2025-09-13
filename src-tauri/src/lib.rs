mod commands;
mod file;
mod demo;
mod terminal;
mod exit_app;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::my_custom_command,
      demo::greet, 
      exit_app::exit,
      terminal::term])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

