use super::interface::ThirdPartyService;
use crate::infrastructure::dto::{RequestBody, ResponseBody, WordInfo, WordMeaning};
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::debug;

pub struct HongliangServiceImpl {
    client: reqwest::Client,
    api_url: String,
}

impl HongliangServiceImpl {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url: "https://pt.hongliang.fun/api/v1/pt".to_string(),
        }
    }
}

#[async_trait]
impl ThirdPartyService for HongliangServiceImpl {
    async fn fetch_word_info(&self, word: &str) -> Result<WordInfo> {
        let body = RequestBody {
            re: true,
            content: word.to_string(),
        };

        let response = self
            .client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .body(json!(body).to_string())
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            return Err(anyhow!("Non-success status code: {}", response.status()));
        }

        let response_text = response
            .text()
            .await
            .context("Failed to get response text")?;

        debug!("response text form  HongLiangService : {}", response_text);

        let response_body: ResponseBody =
            serde_json::from_str(&response_text).context("Failed to parse response")?;

        if response_body.err != 0 {
            return Err(anyhow!("API returned an error: err={}", response_body.err));
        }





        let data = response_body
            .data
            .get(0)
            .ok_or_else(|| anyhow!("No data returned from API"))?;

        if data.len() < 6 {
            return Err(anyhow!("Incomplete data returned from API"));
        }

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

        let meanings_array = data
            .get(5)
            .and_then(|v| v.as_array())
            .ok_or_else(|| anyhow!("Missing or invalid meanings array"))?;

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
            return Err(anyhow!("No valid meanings found"));
        }

        Ok(WordInfo {
            uk_phonetic,
            us_phonetic,
            meanings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_word_info() {
        let service = HongliangServiceImpl::new();
        let result = service.fetch_word_info("name").await;
        assert!(result.is_ok());

        if let Ok(word_info) = result {
            assert!(!word_info.uk_phonetic.is_empty());
            assert!(!word_info.us_phonetic.is_empty());
            assert!(!word_info.meanings.is_empty());
        }
    }
}
