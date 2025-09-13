
// remember to call `.manage(MyState::default())`
#[tauri::command]
pub fn exit(app_handle: tauri::AppHandle) {
  
    app_handle.exit(0); // 0是退出码
}