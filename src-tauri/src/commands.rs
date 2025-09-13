// 参数应该以小驼峰命名，并作为JSON对象的键的传递, 如
// invoke('my_custom_command', { invokeMessage: 'Hello!' });
// invoke 返回一个使用返回值解析的promise
// invoke('my_custom_command').then((message) => console.log(message));
#[tauri::command]
pub fn my_custom_command(invoke_message: String) -> String {
  format!("Hello {}", invoke_message)
}