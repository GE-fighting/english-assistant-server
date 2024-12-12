use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents the request body for the Hongliang API
#[derive(Debug, Serialize)]
pub struct RequestBody {
    re: bool,
    content: String,
}

/// Represents a word meaning with its part of speech and definition
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct WordMeaning {
    pub pos: String,        // Part of speech
    pub definition: String, // Definition of the word
}

/// Represents the complete word information returned by the API
#[derive(Debug, Deserialize)]
pub struct WordInfo {
    pub uk_phonetic: String,
    pub us_phonetic: String,
    pub meanings: Vec<WordMeaning>,
}

/// Represents the API response structure
#[derive(Debug, Deserialize)]
struct ResponseBody {
    err: i32,
    #[serde(default)]
    data: Vec<Vec<serde_json::Value>>,
    content: String,
    duration: f64,
    re_code: String,
}

/// Fetches word information from the Hongliang API
///
/// # Arguments
/// * `word` - The word to look up
///
/// # Returns
/// * `Result<WordInfo>` - The word information including phonetics and meanings
pub async fn fetch_word_info(word: &str) -> Result<WordInfo> {
    let client = reqwest::Client::new();
    let url = "https://pt.hongliang.fun/api/v1/pt";
    let body = RequestBody {
        re: true,
        content: word.to_string(),
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json!(body).to_string())
        .send()
        .await
        .context("Failed to send request")?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Non-success status code: {}",
            response.status()
        ));
    }

    let response_text = response
        .text()
        .await
        .context("Failed to get response text")?;
    let response_body: ResponseBody =
        serde_json::from_str(&response_text).context("Failed to parse response")?;

    if response_body.err != 0 {
        return Err(anyhow::anyhow!(
            "API returned an error: err={}",
            response_body.err
        ));
    }

    let data = response_body
        .data
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("No data returned from API"))?;

    if data.len() < 6 {
        return Err(anyhow::anyhow!("Incomplete data returned from API"));
    }

    // Extract phonetics
    let uk_phonetic = data
        .get(1)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    let us_phonetic = data
        .get(3)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();

    // Extract meanings array
    let meanings_array = data
        .get(5)
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("Missing or invalid meanings array"))?;

    // Parse meanings
    let meanings = meanings_array
        .iter()
        .filter_map(|item| {
            if let Some(arr) = item.as_array() {
                if arr.len() >= 2 {
                    Some(WordMeaning {
                        pos: arr[0].as_str()?.to_string(),
                        definition: arr[1].as_str()?.to_string(),
                    })
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if meanings.is_empty() {
        return Err(anyhow::anyhow!("No valid meanings found"));
    }

    Ok(WordInfo {
        uk_phonetic,
        us_phonetic,
        meanings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_word_info() {
        let result = fetch_word_info("name").await;
        assert!(result.is_ok());

        if let Ok(word_info) = result {
            assert!(!word_info.uk_phonetic.is_empty());
            assert!(!word_info.us_phonetic.is_empty());
            assert!(!word_info.meanings.is_empty());
        }
    }
}
