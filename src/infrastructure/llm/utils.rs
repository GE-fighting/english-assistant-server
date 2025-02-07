use crate::infrastructure::dto::PhoneticsResponse;
use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct SentencePair {
    english: String,
    chinese: String,
}

/// Clean LLM response that might be wrapped in code blocks
pub fn clean_json_response(content: &str) -> String {
    let trimmed = content.trim();

    // If response is wrapped in ```json ... ``` or ``` ... ```
    if trimmed.starts_with("```") {
        let without_start = trimmed
            .trim_start_matches("```json")
            .trim_start_matches("```");
        let without_end = without_start.trim_end_matches("```");
        without_end.trim().to_string()
    } else {
        trimmed.to_string()
    }
}

/// Extract and validate phonetics from LLM response
pub fn extract_phonetics(content: &str) -> Result<(String, String)> {
    let response: PhoneticsResponse = serde_json::from_str(content)
        .map_err(|err| anyhow!("Failed to parse JSON response: {}", err))?;

    let us_phonetic = response.us_ipa.trim().to_string();
    let uk_phonetic = response.uk_ipa.trim().to_string();

    // Validate phonetic format
    if us_phonetic.contains('/') || us_phonetic.contains('[') {
        return Err(anyhow!("Invalid US phonetic format: {}", us_phonetic));
    }
    if uk_phonetic.contains('/') || uk_phonetic.contains('[') {
        return Err(anyhow!("Invalid UK phonetic format: {}", uk_phonetic));
    }

    Ok((us_phonetic, uk_phonetic))
}
/// Extract and validate example sentences from LLM response
pub fn extract_example_sentences(content: &str) -> Result<String> {
    let sentences: Vec<SentencePair> = serde_json::from_str(content)
        .map_err(|err| anyhow!("Failed to parse JSON response: {}", err))?;

    if sentences.is_empty() {
        return Err(anyhow!("No valid sentence pairs found in the response"));
    }
    if sentences.len() != 2 {
        return Err(anyhow!(
            "Expected 2 sentence pairs, got {}",
            sentences.len()
        ));
    }

    let mut examples = String::new();
    for pair in sentences {
        let english = pair.english.trim();
        let chinese = pair.chinese.trim();
        examples.push_str(&format!("{}\n{}\n", english, chinese));
    }
    Ok(examples)
}
