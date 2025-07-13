use serde_json::{json, Value};
use crate::config::Config;

fn prepare_payload(config: Config, prompt: &str, content: &str) -> Value {
    let user_prompt = format!("Process this code edition request. In your answer, return the edited code and the explanation about the changes performed on it:\n
        -------\n
        {}:\n
        -------\n
        ```{}```", prompt, content);
    let result = json!({
        "model": config.model,
        "prompt": user_prompt,
        "system": config.system_prompt,
        "temperature": 0.1
    });
    println!("Prepared payload: {}", result);
    result
}

fn request(prompt: &str, content: &str) -> Result<String, String> {
    let config = Config::load("config.json")?;
    let url = format!("{}/api/generate", config.server_address);
    println!("Requesting URL: {}", url);
    let payload = prepare_payload(config, prompt, content);
        
    let response = ureq::post(&url)
        .header("Content-Type", "application/json")
        .send_json(payload).map_err(|e| format!("Request failed: {}", e))?
        .body_mut()
        .read_to_string();
    match response {
        Ok(body) => {
            println!("Response body: {}", body);
            Ok(body)
        },
        Err(e) => Err(format!("Failed to read response: {}", e)),
    }
}

pub fn generate(prompt: &str, content: &str) -> Result<String, String> {
    let response = request(prompt, content)?;
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
    println!("Generated content: {}", content);
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
        let prompt = "Fix this code in Python";
        let content = "print('Hello world'')')";
        let result = request(prompt, content);
        if result.is_err() {
            eprintln!("Request failed: {}", result.as_ref().err().unwrap());
        }
        assert!(result.is_ok(), "Request should succeed");
        let response = result.unwrap();
        assert!(!response.is_empty(), "Response should not be empty");
    }

    #[test]
    fn test_generate() {
        let prompt = "Fix this code in Python";
        let content = "print('Hello world'')')";
        let result = generate(prompt, content);
        if result.is_err() {
            eprintln!("Generate failed: {}", result.as_ref().err().unwrap());
        }
        assert!(result.is_ok(), "Generate should succeed");
        let response = result.unwrap();
        assert!(!response.is_empty(), "Generated content should not be empty");
    }
}
