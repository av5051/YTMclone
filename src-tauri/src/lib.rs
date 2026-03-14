use serde;
use reqwest;
use tauri::http::response;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[derive(serde::Serialize , serde::Deserialize)]
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
async fn search_music(query: &str) -> Result<Vec<Song> , String > {

    let url  = " https://jsonplaceholder.typicode.com/posts";
    let response = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?;
    let data: Vec<Song> = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(data)

}

#[tauri::command]
fn get_streaming_url(id : String) -> String{

    println!("Fetching stream for ID: {}", id);
    
    "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3".to_string()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet , search_music, get_streaming_url])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
