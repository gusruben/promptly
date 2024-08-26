// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use misanthropy::{Anthropic, Content, ContentBlockDelta, MessagesRequest, StreamEvent};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![prompt_claude])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn prompt_claude(window: tauri::Window, prompt: String) -> Result<(), String> {
    let client = Anthropic::from_env().map_err(|e| e.to_string())?;

    let mut request = MessagesRequest::default();
    request.add_user(Content::text(&prompt));
    request.stream = true;

    let mut stream = client.messages_stream(&request).map_err(|e| e.to_string())?;
	
    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                match event {
                    StreamEvent::ContentBlockDelta { delta, .. } => {
                        if let ContentBlockDelta::TextDelta { text } = delta {
                            window.emit("response", &text).map_err(|e| e.to_string())?;
                        }
                    },
                    StreamEvent::MessageStop => {
                        window.emit("response-end", ()).map_err(|e| e.to_string())?;
                    },
                    _ => {} // ignore other events
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(())
}
