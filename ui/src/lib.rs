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
#[serde(tag = "type", rename_all = "camelCase")]
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
    #[serde(rename = "wasm")]
    WasmUI { mount_script: String },
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

fn create_metadata() -> PluginMetadata {
    PluginMetadata {
        name: "Example Plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A sample plugin demonstrating UI capabilities".to_string(),
        author: "Plugin Developer".to_string(),
        logo: Some("data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEyIDJMMTMuMDkgOC4yNkwyMCA9TDEzLjA5IDE1Ljc0TDEyIDIyTDEwLjkxIDE1Ljc0TDQgOUwxMC45MSA4LjI2TDEyIDJaIiBmaWxsPSIjRkY2NTAwIi8+Cjwvc3ZnPgo=".to_string()),
        license: Some("MIT".to_string()),
        homepage: Some("https://example.com".to_string()),
        repository: Some("https://github.com/example/plugin".to_string()),
        keywords: vec!["example".to_string(), "ui".to_string(), "demo".to_string()],
        api_version: "1.0".to_string(),
    }
}

#[plugin_fn]
pub fn get_manifest(_input: String) -> FnResult<String> {
    let metadata = create_metadata();
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
    Ok(serde_json::to_string(&manifest).unwrap_or_else(|_| "{}".to_string()))
}

#[plugin_fn]
pub fn get_metadata(_input: String) -> FnResult<String> {
    let metadata = create_metadata();
    Ok(serde_json::to_string(&metadata).unwrap_or_else(|_| "{}".to_string()))
}

#[plugin_fn]
pub fn get_ui(_input: String) -> FnResult<String> {
    let ui = UIElement::Container {
        id: Some("root".to_string()),
        direction: "column".to_string(),
        style: None,
        children: vec![
            UIElement::Text {
                id: "welcome".to_string(),
                value: "Welcome to the Example Plugin!".to_string(),
                style: None,
            },
            UIElement::Input {
                id: "input".to_string(),
                placeholder: Some("Type something...".to_string()),
                value: None,
                input_type: Some("text".to_string()),
                on_change: Some("handle_input_change".to_string()),
                style: None,
            },
            UIElement::Select {
                id: "mode".to_string(),
                options: vec![
                    SelectOption {
                        value: "light".to_string(),
                        label: "Light Mode".to_string(),
                    },
                    SelectOption {
                        value: "dark".to_string(),
                        label: "Dark Mode".to_string(),
                    },
                ],
                selected: Some("light".to_string()),
                on_change: Some("handle_select_change".to_string()),
                style: None,
            },
            UIElement::Button {
                id: "click".to_string(),
                label: "Click Me".to_string(),
                on_click: "handle_click".to_string(),
                disabled: None,
                style: None,
            },
        ],
    };
    Ok(serde_json::to_string(&ui).unwrap_or_else(|_| "{}".to_string()))
}

#[plugin_fn]
pub fn handle_event(input: String) -> FnResult<String> {
    let event: EventPayload = match serde_json::from_str(&input) {
        Ok(event) => event,
        Err(_) => {
            let response = EventResponse {
                updates: vec![],
                state: None,
                notifications: Some(vec![Notification {
                    level: "error".to_string(),
                    message: "Failed to parse event".to_string(),
                    duration: Some(3000),
                }]),
            };
            return Ok(serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()));
        }
    };
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
            if let Some(data) = &event.data {
                if let Some(value) = data.get("value") {
                    if let Some(text) = value.as_str() {
                        updates.push(UIUpdate {
                            target_id: "welcome".to_string(),
                            update_type: "replace".to_string(),
                            data: serde_json::json!({
                                "value": format!("You typed: {}", text)
                            }),
                        });
                    }
                }
            }
        }
        "handle_select_change" => {
            if let Some(data) = &event.data {
                if let Some(value) = data.get("value") {
                    if let Some(mode_str) = value.as_str() {
                        updates.push(UIUpdate {
                            target_id: "welcome".to_string(),
                            update_type: "replace".to_string(),
                            data: serde_json::json!({
                                "value": format!("Mode changed to: {}", mode_str)
                            }),
                        });
                        notifications.push(Notification {
                            level: "info".to_string(),
                            message: format!("Switched to {} mode", mode_str),
                            duration: Some(2000),
                        });
                    }
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
            "event_count": 1
        })),
        notifications: if notifications.is_empty() {
            None
        } else {
            Some(notifications)
        },
    };
    Ok(serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string()))
}
