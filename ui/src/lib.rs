use extism_pdk::*;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub logo: Option<String>,
    pub license: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub api_version: String,
}
#[derive(Serialize, Deserialize)]
pub struct PluginManifest {
    pub metadata: PluginMetadata,
    pub capabilities: Vec<String>,
    pub ui_available: bool,
    pub event_handlers: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum UIElement {
    #[serde(rename = "container")]
    Container {
        id: Option<String>,
        direction: String,
        style: Option<serde_json::Value>,
        children: Vec<UIElement>,
    },
    #[serde(rename = "text")]
    Text {
        id: String,
        value: String,
        style: Option<serde_json::Value>,
    },
    #[serde(rename = "button")]
    Button {
        id: String,
        label: String,
        #[serde(rename = "onClick")]
        on_click: String,
        disabled: Option<bool>,
        style: Option<serde_json::Value>,
    },
    #[serde(rename = "input")]
    Input {
        id: String,
        placeholder: Option<String>,
        value: Option<String>,
        input_type: Option<String>,
        #[serde(rename = "onChange")]
        on_change: Option<String>,
        style: Option<serde_json::Value>,
    },
    #[serde(rename = "select")]
    Select {
        id: String,
        options: Vec<SelectOption>,
        selected: Option<String>,
        #[serde(rename = "onChange")]
        on_change: Option<String>,
        style: Option<serde_json::Value>,
    },
}
#[derive(Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}
#[derive(Serialize, Deserialize)]
pub struct UIResponse {
    pub ui: UIElement,
    pub state: Option<serde_json::Value>,
}
#[derive(Deserialize)]
pub struct EventPayload {
    pub id: String,
    pub event_type: String,
    pub data: Option<serde_json::Value>,
}
#[derive(Serialize)]
pub struct EventResponse {
    pub updates: Vec<UIUpdate>,
    pub state: Option<serde_json::Value>,
    pub notifications: Option<Vec<Notification>>,
}
#[derive(Serialize)]
pub struct UIUpdate {
    pub target_id: String,
    pub update_type: String,
    pub data: serde_json::Value,
}
#[derive(Serialize)]
pub struct Notification {
    pub level: String,
    pub message: String,
    pub duration: Option<u32>,
}
#[plugin_fn]
pub fn get_manifest(_input: String) -> FnResult<String> {
    let metadata = PluginMetadata {
        name: "Example Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A sample plugin demonstrating UI capabilities".to_string(),
        author: "Plugin Developer".to_string(),
        logo: Some("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA5TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjRkY2NTAwIi8+Cjwvc3ZnPgo=".to_string()),
        license: Some("MIT".to_string()),
        homepage: Some("https://example.com".to_string()),
        repository: Some("https://github.com/example/plugin".to_string()),
        keywords: vec!["example".to_string(), "ui".to_string(), "demo".to_string()],
        api_version: "1.0".to_string(),
    };
    let manifest = PluginManifest {
        metadata,
        capabilities: vec![
            "ui".to_string(),
            "events".to_string(),
            "state_management".to_string(),
        ],
        ui_available: true,
        event_handlers: vec![
            "handle_click".to_string(),
            "handle_input_change".to_string(),
            "handle_select_change".to_string(),
        ],
    };
    Ok(serde_json::to_string(&manifest)?)
}
#[plugin_fn]
pub fn get_metadata(_input: String) -> FnResult<String> {
    let metadata = PluginMetadata {
        name: "Example Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A sample plugin demonstrating UI capabilities".to_string(),
        author: "Plugin Developer".to_string(),
        logo: Some("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA5TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjRkY2NTAwIi8+Cjwvc3ZnPgo=".to_string()),
        license: Some("MIT".to_string()),
        homepage: Some("https://example.com".to_string()),
        repository: Some("https://github.com/example/plugin".to_string()),
        keywords: vec!["example".to_string(), "ui".to_string(), "demo".to_string()],
        api_version: "1.0".to_string(),
    };
    Ok(serde_json::to_string(&metadata)?)
}
#[plugin_fn]
pub fn get_ui(_input: String) -> FnResult<String> {
    let ui = UIElement::Container {
        id: Some("main_container".to_string()),
        direction: "column".to_string(),
        style: Some(serde_json::json!({
            "padding": "16px",
            "gap": "12px",
            "backgroundColor": "#f5f5f5",
            "borderRadius": "8px"
        })),
        children: vec![
            UIElement::Text {
                id: "welcome".to_string(),
                value: "Hello from Example Plugin!".to_string(),
                style: Some(serde_json::json!({
                    "fontSize": "18px",
                    "fontWeight": "bold",
                    "color": "#333"
                })),
            },
            UIElement::Input {
                id: "user_input".to_string(),
                placeholder: Some("Enter your message...".to_string()),
                value: None,
                input_type: Some("text".to_string()),
                on_change: Some("handle_input_change".to_string()),
                style: Some(serde_json::json!({
                    "padding": "8px",
                    "borderRadius": "4px",
                    "border": "1px solid #ddd"
                })),
            },
            UIElement::Select {
                id: "mode_select".to_string(),
                options: vec![
                    SelectOption {
                        value: "casual".to_string(),
                        label: "Casual Mode".to_string(),
                    },
                    SelectOption {
                        value: "formal".to_string(),
                        label: "Formal Mode".to_string(),
                    },
                    SelectOption {
                        value: "creative".to_string(),
                        label: "Creative Mode".to_string(),
                    },
                ],
                selected: Some("casual".to_string()),
                on_change: Some("handle_select_change".to_string()),
                style: Some(serde_json::json!({
                    "padding": "8px",
                    "borderRadius": "4px",
                    "border": "1px solid #ddd"
                })),
            },
            UIElement::Button {
                id: "submit_btn".to_string(),
                label: "Submit".to_string(),
                on_click: "handle_click".to_string(),
                disabled: Some(false),
                style: Some(serde_json::json!({
                    "padding": "10px 20px",
                    "backgroundColor": "#007bff",
                    "color": "white",
                    "border": "none",
                    "borderRadius": "4px",
                    "cursor": "pointer"
                })),
            },
        ],
    };
    let response = UIResponse {
        ui,
        state: Some(serde_json::json!({
            "current_mode": "casual",
            "user_input": "",
            "click_count": 0
        })),
    };
    Ok(serde_json::to_string(&response)?)
}
#[plugin_fn]
pub fn handle_event(input: String) -> FnResult<String> {
    let event: EventPayload = serde_json::from_str(&input)?;
    let mut updates = Vec::new();
    let mut notifications = Vec::new();
    match event.id.as_str() {
        "handle_click" => {
            updates.push(UIUpdate {
                target_id: "welcome".to_string(),
                update_type: "replace".to_string(),
                data: serde_json::json!({
                    "value": "Button clicked! Thank you for trying the plugin."
                }),
            });
            notifications.push(Notification {
                level: "success".to_string(),
                message: "Button clicked successfully!".to_string(),
                duration: Some(3000),
            });
        }
        "handle_input_change" => {
            if let Some(data) = event.data {
                if let Some(value) = data.get("value") {
                    updates.push(UIUpdate {
                        target_id: "welcome".to_string(),
                        update_type: "replace".to_string(),
                        data: serde_json::json!({
                            "value": format!("You typed: {}", value.as_str().unwrap_or(""))
                        }),
                    });
                }
            }
        }
        "handle_select_change" => {
            if let Some(data) = event.data {
                if let Some(value) = data.get("value") {
                    updates.push(UIUpdate {
                        target_id: "welcome".to_string(),
                        update_type: "replace".to_string(),
                        data: serde_json::json!({
                            "value": format!("Mode changed to: {}", value.as_str().unwrap_or(""))
                        }),
                    });
                    notifications.push(Notification {
                        level: "info".to_string(),
                        message: format!("Switched to {} mode", value.as_str().unwrap_or("")),
                        duration: Some(2000),
                    });
                }
            }
        }
        _ => {
            notifications.push(Notification {
                level: "warning".to_string(),
                message: format!("Unknown event: {}", event.id),
                duration: Some(2000),
            });
        }
    }
    let response = EventResponse {
        updates,
        state: Some(serde_json::json!({
            "last_event": event.id,
            "timestamp": chrono::Utc::now().timestamp()
        })),
        notifications: if notifications.is_empty() {
            None
        } else {
            Some(notifications)
        },
    };
    Ok(serde_json::to_string(&response)?)
}
