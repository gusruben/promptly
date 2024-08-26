// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use misanthropy::{Anthropic, Content, ContentBlockDelta, MessagesRequest, StreamEvent, Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use anthropic_sdk::{Client, ToolChoice};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![prompt_claude])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
struct BreakDown {
    task: String,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug)]
struct RunCode {
    code: String,
}

#[tauri::command]
async fn prompt_claude(
    window: tauri::Window,
    systemPrompt: String,
    prompt: String,
    tools: Vec<String>,
) -> Result<(), String> {
    let client = Anthropic::from_env().map_err(|e| e.to_string())?;
    
    println!("System prompt: {}", prompt.clone());
    println!("Tools: {:?}", tools.clone());
    let mut request = MessagesRequest::default()
        .with_model("claude-3-5-sonnet-20240620".to_string())
        .with_system(systemPrompt);

    if tools.contains(&"break_down".to_string()) {
        let break_down_tool = Tool::new::<BreakDown>();
        request = request.with_tool(break_down_tool);
    }

    if tools.contains(&"run_code".to_string()) {
        let run_code_tool = Tool::new::<RunCode>();
        // request = request.with_tool(run_code_tool);
    }

    request.add_user(Content::text(&prompt));


    request.stream = true;
    println!("Request: {:?}", request);
    let mut stream = client
        .messages_stream(&request)
        .map_err(|e| e.to_string())?;

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                match event {
                    StreamEvent::ContentBlockDelta { delta, .. } => {
                        if let ContentBlockDelta::TextDelta { text } = delta {
                            window.emit("response", &text).map_err(|e| e.to_string())?;
                        }
                    }
                    StreamEvent::MessageStop => {
                        window.emit("response-end", ()).map_err(|e| e.to_string())?;
                    }
                    _ => {} // ignore other events
                }
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    // check for tool use in the response
    for content in request.messages.last().unwrap().content.iter() {
        if let Content::ToolUse(tool_use) = content {
            match tool_use.name.as_str() {
                "BreakDown" => {
                    if let Ok(break_down) =
                        serde_json::from_value::<BreakDown>(tool_use.input.clone())
                    {
                        window
                            .emit("tool-break-down", &break_down.task)
                            .map_err(|e| e.to_string())?;
                    }
                }
                "RunCode" => {}
                _ => {}
            }
        }
    }

    Ok(())
}
