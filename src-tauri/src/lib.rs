use serde::Serialize;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(Serialize)]
struct Song{
    title : String,
    artist : String,
    id :  String,
}


#[tauri::command]
fn greet(name: &str) -> String {
    println!("bhosdike");
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn search_music(query: &str) -> Vec<Song>{
    let song1  = Song{
        title : "song1".to_string(),
        artist : "arpit".to_string(),
        id : "1".to_string()
    };
    let song2  = Song{
        title : "song2".to_string(),
        artist : "arpit".to_string(),
        id : "2".to_string()
    };
    vec![song1,song2]
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet , search_music])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
