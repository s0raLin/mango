use rand::Rng;

// 猜数字
#[tauri::command]
pub fn greet(num: i32)->Result<String, String> {
    
    
    let mut rng = rand::thread_rng();
    let rang = rng.gen_range(1..100);
    

    if num > rang {
        Err("猜大了".to_string())
    } else if num < rang {
        Err("猜小了".to_string())
    } else {
        Ok("恭喜你猜对了".to_string())
    }
}