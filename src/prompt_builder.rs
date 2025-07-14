use std::fs;
use std::io;

pub fn build_prompt(filename: &str, user_prompt: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(filename)?;
    let file_extension = filename
        .rsplit('.')
        .next()
        .unwrap_or("");
    let prompt = format!(
        "Based on the following request, review the include code.\n\
         Response with:\n\
         (a) explanation of the problems detected and changes realized\n\
         (b) the modified code.\n\n\
         Keep both parts separated by a line.\n
         Don't include any other text.\n\n\
         Request: {}\n\
         ```{}\n{}\n```\n",
         user_prompt, file_extension, content);
    Ok(prompt)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_build_prompt() {
        let filename = "config-example.json";
        let user_prompt = "Please review the code for any issues.";
        let result = build_prompt(filename, user_prompt);
        println!("Prompt: {}", result.as_ref().unwrap());
        assert!(result.is_ok());
        let prompt = result.unwrap();
        assert!(prompt.contains("Based on the following request, review the include code."));
        assert!(prompt.contains("Request: Please review the code for any issues."));
        assert!(prompt.contains("```json"));
        assert!(prompt.contains("```"));
    }
}
