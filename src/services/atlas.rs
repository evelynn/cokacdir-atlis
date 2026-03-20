use std::io::BufRead;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use serde_json::Value;

use super::claude::{StreamMessage, CancelToken, debug_log};

/// AI Atlas API configuration
pub struct AtlasConfig {
    pub api_key: String,
    pub agent_id: String,
    pub base_url: String,
}

impl AtlasConfig {
    /// Load config from environment variables, fall back to settings file
    pub fn from_env() -> Result<Self, String> {
        let api_key = std::env::var("ATLAS_API_KEY").ok();
        let agent_id = std::env::var("ATLAS_AGENT_ID").ok();
        let base_url = std::env::var("ATLAS_BASE_URL")
            .unwrap_or_else(|_| "https://ai-atlas.hansol.net/api/v1/public".to_string());

        // Try settings file if env vars not set
        let (file_key, file_agent) = load_from_settings_file();

        let api_key = api_key.or(file_key)
            .ok_or_else(|| "ATLAS_API_KEY not set. Set env var or use settings file (~/.ai-atlas-tui/settings.json)".to_string())?;
        let agent_id = agent_id.or(file_agent)
            .ok_or_else(|| "ATLAS_AGENT_ID not set. Set env var or use settings file (~/.ai-atlas-tui/settings.json)".to_string())?;

        if api_key.is_empty() || agent_id.is_empty() {
            return Err("ATLAS_API_KEY and ATLAS_AGENT_ID must not be empty".to_string());
        }

        Ok(Self { api_key, agent_id, base_url })
    }
}

/// Load API key and agent ID from settings file
fn load_from_settings_file() -> (Option<String>, Option<String>) {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return (None, None),
    };
    let path = home.join(".ai-atlas-tui").join("settings.json");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return (None, None),
    };
    let json: Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return (None, None),
    };
    let api_key = json.get("atlas_api_key").and_then(|v| v.as_str()).map(String::from);
    let agent_id = json.get("atlas_agent_id").and_then(|v| v.as_str()).map(String::from);
    (api_key, agent_id)
}

/// Check if AI Atlas is available (config exists)
pub fn is_atlas_available() -> bool {
    AtlasConfig::from_env().is_ok()
}

/// Save Atlas API key and Agent ID to settings file
pub fn save_atlas_config(api_key: &str, agent_id: &str) -> Result<(), String> {
    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let config_dir = home.join(".ai-atlas-tui");
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config dir: {}", e))?;

    let path = config_dir.join("settings.json");

    // Load existing settings or create new
    let mut json: serde_json::Map<String, Value> = if let Ok(content) = std::fs::read_to_string(&path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::Map::new()
    };

    json.insert("atlas_api_key".to_string(), Value::String(api_key.to_string()));
    json.insert("atlas_agent_id".to_string(), Value::String(agent_id.to_string()));

    let content = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("JSON serialize error: {}", e))?;
    std::fs::write(&path, content)
        .map_err(|e| format!("Failed to write settings: {}", e))?;

    Ok(())
}

/// Get current Atlas config info for display (masked key)
pub fn get_atlas_config_display() -> (String, String, String) {
    let (file_key, file_agent) = load_from_settings_file();
    let env_key = std::env::var("ATLAS_API_KEY").ok();
    let env_agent = std::env::var("ATLAS_AGENT_ID").ok();

    let api_key = env_key.or(file_key).unwrap_or_default();
    let agent_id = env_agent.or(file_agent).unwrap_or_default();

    let masked_key = if api_key.len() > 12 {
        format!("****{}", &api_key[api_key.len()-8..])
    } else if api_key.is_empty() {
        "(not set)".to_string()
    } else {
        "****".to_string()
    };

    let display_agent = if agent_id.is_empty() {
        "(not set)".to_string()
    } else {
        agent_id
    };

    let source = if std::env::var("ATLAS_API_KEY").is_ok() {
        "env".to_string()
    } else {
        let home = dirs::home_dir().map(|h| h.join(".ai-atlas-tui").join("settings.json").display().to_string()).unwrap_or_default();
        home
    };

    (masked_key, display_agent, source)
}

/// Create a new chat session
pub fn create_session(config: &AtlasConfig, title: &str) -> Result<String, String> {
    debug_log(&format!("atlas: Creating session with title: {}", title));

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let url = format!("{}/agents/{}/sessions", config.base_url, config.agent_id);

    let response = client
        .post(&url)
        .header("x-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"title": title}).to_string())
        .send()
        .map_err(|e| format!("Session creation failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("Session creation error {}: {}", status, body));
    }

    let json: Value = response.json().map_err(|e| format!("JSON parse error: {}", e))?;
    let session_id = json.get("id")
        .and_then(|v: &Value| v.as_str())
        .ok_or_else(|| "No session ID in response".to_string())?;

    debug_log(&format!("atlas: Session created: {}", session_id));
    Ok(session_id.to_string())
}

/// Execute a streaming message request to AI Atlas API.
/// Compatible with claude::execute_command_streaming signature.
pub fn execute_command_streaming(
    prompt: &str,
    session_id: Option<&str>,
    _working_dir: &str,
    sender: Sender<StreamMessage>,
    _system_prompt: Option<&str>,
    _allowed_tools: Option<&[String]>,
    cancel_token: Option<Arc<CancelToken>>,
    _model: Option<&str>,
    _no_session_persistence: bool,
) -> Result<(), String> {
    debug_log("atlas: execute_command_streaming called");

    let config = AtlasConfig::from_env()?;

    // Ensure we have a session
    let sid = match session_id {
        Some(id) if !id.is_empty() => id.to_string(),
        _ => {
            let new_sid = create_session(&config, "TUI Chat")?;
            let _ = sender.send(StreamMessage::Init { session_id: new_sid.clone() });
            new_sid
        }
    };

    debug_log(&format!("atlas: Using session: {}", sid));

    // Build HTTP request for SSE streaming
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(300))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let url = format!("{}/agents/{}/sessions/{}/messages/stream",
        config.base_url, config.agent_id, sid);

    let response = client
        .post(&url)
        .header("x-api-key", &config.api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::json!({"message": prompt}).to_string())
        .send()
        .map_err(|e| format!("Streaming request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_default();
        return Err(format!("API error {}: {}", status, body));
    }

    // Parse SSE stream
    parse_sse_stream(response, &sender, &cancel_token, &sid)
}

/// Parse SSE (Server-Sent Events) stream and convert to StreamMessage
fn parse_sse_stream(
    response: reqwest::blocking::Response,
    sender: &Sender<StreamMessage>,
    cancel_token: &Option<Arc<CancelToken>>,
    session_id: &str,
) -> Result<(), String> {
    let reader = std::io::BufReader::new(response);
    let mut accumulated_text = String::new();
    let mut event_type = String::new();
    let mut data_buf = String::new();

    for line_result in reader.lines() {
        // Check cancellation
        if let Some(ref token) = cancel_token {
            if token.cancelled.load(std::sync::atomic::Ordering::Relaxed) {
                debug_log("atlas: Cancelled by user");
                return Ok(());
            }
        }

        let line: String = match line_result {
            Ok(l) => l,
            Err(e) => {
                debug_log(&format!("atlas: Read error: {}", e));
                break;
            }
        };

        if line.starts_with("event: ") {
            event_type = line[7..].trim().to_string();
        } else if line.starts_with("data: ") {
            data_buf = line[6..].to_string();
        } else if line.is_empty() && !data_buf.is_empty() {
            // Empty line = event boundary, process accumulated event
            process_sse_event(&event_type, &data_buf, sender, &mut accumulated_text);
            event_type.clear();
            data_buf.clear();
        }
    }

    // Send Done if not already sent
    let _ = sender.send(StreamMessage::Done {
        result: "end_turn".to_string(),
        session_id: Some(session_id.to_string()),
    });

    debug_log("atlas: SSE stream completed");
    Ok(())
}

/// Process a single SSE event and send as StreamMessage
fn process_sse_event(
    event_type: &str,
    data: &str,
    sender: &Sender<StreamMessage>,
    accumulated_text: &mut String,
) {
    let parsed: Value = match serde_json::from_str(data) {
        Ok(v) => v,
        Err(_) => return,
    };

    let status = parsed.get("status").and_then(|v| v.as_str()).unwrap_or("");
    let content = parsed.get("content").and_then(|v| v.as_str()).unwrap_or("");

    match (event_type, status) {
        ("done", _) | (_, "end") => {
            // Done event — don't send here, let parse_sse_stream handle it
        }
        ("error", _) => {
            let _ = sender.send(StreamMessage::Error {
                message: content.to_string(),
                stdout: String::new(),
                stderr: String::new(),
                exit_code: None,
            });
        }
        (_, "streaming") => {
            accumulated_text.push_str(content);
            // Send accumulated text (ai_screen expects full text replacement, not delta)
            let _ = sender.send(StreamMessage::Text {
                content: accumulated_text.clone(),
            });
        }
        (_, "notice") | (_, "info") => {
            // System notifications — prepend to accumulated text
            if !content.is_empty() {
                let notice = format!("[{}]\n", content);
                accumulated_text.push_str(&notice);
                let _ = sender.send(StreamMessage::Text {
                    content: accumulated_text.clone(),
                });
            }
        }
        _ => {}
    }
}
