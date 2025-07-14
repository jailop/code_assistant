use serde_json::{json, Value};
use crate::config::Config;

fn prepare_payload(config: Config, prompt: &str) -> Value {
    let result = json!({
        "model": config.model,
        "prompt": prompt,
        "system": config.system_prompt,
        "temperature": config.temperature.unwrap_or(0.1), 
    });
    result
}

fn request(config: Config, prompt: &str) -> Result<String, String> {
    let url = format!("{}/api/generate", config.server_address);
    let payload = prepare_payload(config, prompt);
    let request = ureq::post(&url)
        .header("Content-Type", "application/json")
        .send_json(payload);
    let resp = match request {
        Ok(mut response) => response
            .body_mut()
            .read_to_string(),
        Err(e) => return Err(format!("Request failed: {}", e)),
    };
    match resp {
        Ok(body) => {
            Ok(body)
        },
        Err(e) => Err(format!("Failed to read response: {}", e)),
    }
}

pub fn generate(config: Config, prompt: &str) -> Result<String, String> {
    let response = request(config, prompt)?;
    let parts = response.split("\n").collect::<Vec<&str>>();
    let mut content = String::new();
    for part in parts {
        if !part.is_empty() {
            let deserialized: Value = serde_json::from_str(part)
                .map_err(|e| format!("Failed to deserialize response part: {}", e))?;
            if let Some(text) = deserialized.get("response") {
                content.push_str(text.as_str().unwrap_or(""));
            }
        }
    }
    if content.is_empty() {
        Err("No content generated".to_string())
    } else {
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_request() {
        let config_path = "config-example.json";
        let config = Config::load(config_path).expect("Failed to load config");
        let prompt = "Just reply: done";
        let result = request(config, &prompt);
        if let Err(e) = &result {
            eprintln!("Request failed: {}", e);
        }
        assert!(result.is_ok(), "Request should succeed");
        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
    }

    #[test]
    fn test_generate() {
        let prompt = "Just reply: done";
        let config_path = "config-example.json";
        let config = Config::load(config_path).expect("Failed to load config");
        let result = generate(config, prompt);
        assert!(result.is_ok(), "Generate should succeed");
        let response = result.unwrap();
        assert!(!response.is_empty(), "Generated content should not be empty");
    }
}
