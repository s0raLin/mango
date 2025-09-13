use std::{process::Command};


#[tauri::command]
pub fn term(command: String, args: Vec<String>) -> Result<String, String> {
    let output = Command::new(&command)
        .args(&args)
        .output();

    let out = match output {
        Ok(output) => {
            output.stdout
        }
        Err(e) => {
            return Err(format!("{}", e))
        }
    };
    let Ok(result) = String::from_utf8(out) else {
        return Err("转换失败".into())
    };
    
    Ok(result)
    
}
