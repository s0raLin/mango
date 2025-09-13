use tauri::ipc::Response;
#[tauri::command]
pub fn read_file() -> Response {
  let data = std::fs::read("/home/cangli").unwrap();
  tauri::ipc::Response::new(data)
}