use crate::infrastructure::dto::WordInfo;
use crate::infrastructure::llm::interface::LLMService;
use crate::infrastructure::llm::prompts::LanguagePrompts;
use crate::infrastructure::llm::provider::LLMConfig;
use crate::infrastructure::llm::utils;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use tracing::debug;

pub struct YiServiceImpl {
    client: Client,
    api_key: String,
    api_url: String,
    timeout: u64,
}

impl YiServiceImpl {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: String::new(),
            api_url: "https://api.lingyiwanwu.com/v1/chat/completions".to_string(),
            timeout: 30,
        }
    }
}

#[async_trait]
impl LLMService for YiServiceImpl {
    fn configure(&mut self, config: &LLMConfig) -> Result<()> {
        if let Some(api_key) = &config.api_key {
            self.api_key = api_key.clone();
        } else {
            return Err(anyhow!("API key is required for Yi service"));
        }

        if let Some(base_url) = &config.base_url {
            self.api_url = base_url.clone();
        }

        if let Some(timeout) = config.timeout {
            self.timeout = timeout;
        }

        Ok(())
    }

    async fn get_phonetics(&self, word: &str) -> Result<(String, String)> {
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "model": "yi-lightning",
                    "messages": [
                        {"role": "system", "content": LanguagePrompts::PHONETICS_SYSTEM},
                        {"role": "user", "content": LanguagePrompts::phonetics_user(word)}
                    ],
                    "temperature": 0.3
                })
                .to_string(),
            )
            .send()
            .await?;

        let response_body: Value = serde_json::from_str(&response.text().await?)?;
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to extract content from AI response"))?;

        utils::extract_phonetics(content)
    }

    async fn get_example_sentences(&self, word: &str) -> Result<String> {
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "model": "yi-lightning",
                    "messages": [
                        {"role": "system", "content": LanguagePrompts::SENTENCES_SYSTEM},
                        {"role": "user", "content": LanguagePrompts::example_sentences_user(word)}
                    ],
                    "temperature": 0.7
                })
                .to_string(),
            )
            .send()
            .await?;

        let response_body: Value = serde_json::from_str(&response.text().await?)?;
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to extract content from AI response"))?;
        let cleaned_content = utils::clean_json_response(content);
        debug!("example sentences from AI response : {}", cleaned_content);
        utils::extract_example_sentences(cleaned_content.as_str())
    }

    async fn get_word_info(&self, word: &str) -> Result<WordInfo> {
        let response = self
            .client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(
                json!({
                    "model": "yi-lightning",
                    "messages": [
                        {"role": "system", "content": LanguagePrompts::WORD_INFO_SYSTEM},
                        {"role": "user", "content": LanguagePrompts::word_info_user(word)}
                    ],
                    "temperature": 0.3
                })
                .to_string(),
            )
            .send()
            .await?;

        let response_body: Value = serde_json::from_str(&response.text().await?)?;
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to extract content from AI response"))?;

        // Clean the response before parsing
        let cleaned_content = utils::clean_json_response(content);
        debug!("word info from AI response : {}", cleaned_content);
        serde_json::from_str::<WordInfo>(&cleaned_content).map_err(|e| {
            anyhow!(
                "Failed to parse word info response for '{}': {}.\nResponse: {}",
                word,
                e,
                cleaned_content
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_example_sentences() {
        let mut service = YiServiceImpl::new();
        service.configure(
            &LLMConfig::new(crate::infrastructure::llm::provider::LLMProvider::Yi)
                .with_api_key(std::env::var("YI_API_KEY").unwrap_or_default()),
        )?;

        let word = "happy";

        match service.get_example_sentences(word).await {
            Ok(sentences) => {
                assert_eq!(sentences.len(), 2, "Should return exactly 2 sentences");

                println!("\nTest results for word '{}':", word);
                println!("Simple sentence:");
                println!("English: {}", sentences[0].0);
                println!("Chinese: {}", sentences[0].1);
                println!("\nComplex sentence:");
                println!("English: {}", sentences[1].0);
                println!("Chinese: {}", sentences[1].1);

                for (eng, _) in &sentences {
                    assert!(
                        eng.to_lowercase().contains(&word.to_lowercase()),
                        "Sentence should contain the word '{}': {}",
                        word,
                        eng
                    );
                }

                for (_, chn) in &sentences {
                    assert!(!chn.is_empty(), "Chinese translation should not be empty");
                    assert!(
                        chn.chars()
                            .any(|c| (c as u32) > 0x4E00 && (c as u32) < 0x9FFF),
                        "Translation should contain Chinese characters: {}",
                        chn
                    );
                }
            }
            Err(e) => panic!("Failed to get example sentences: {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_phonetics() {
        let mut service = YiServiceImpl::new();
        service.configure(
            &LLMConfig::new(crate::infrastructure::llm::provider::LLMProvider::Yi)
                .with_api_key(std::env::var("YI_API_KEY").unwrap_or_default()),
        )?;

        let word = "hello";

        match service.get_phonetics(word).await {
            Ok((us, uk)) => {
                println!("\nTest results for word '{}':", word);
                println!("US phonetic: {}", us);
                println!("UK phonetic: {}", uk);

                assert!(!us.is_empty(), "US phonetic should not be empty");
                assert!(!uk.is_empty(), "UK phonetic should not be empty");

                assert!(
                    us.contains('/') || us.contains('['),
                    "US phonetic should contain phonetic symbols: {}",
                    us
                );
                assert!(
                    uk.contains('/') || uk.contains('['),
                    "UK phonetic should contain phonetic symbols: {}",
                    uk
                );
            }
            Err(e) => panic!("Failed to get phonetics: {}", e),
        }
    }

    #[tokio::test]
    async fn test_get_word_info() {
        println!("\n=== Testing get_word_info ===");

        let mut service = YiServiceImpl::new();
        service.configure(
            &LLMConfig::new(crate::infrastructure::llm::provider::LLMProvider::Yi)
                .with_api_key(std::env::var("YI_API_KEY").unwrap_or_default()),
        )?;

        let word = "name"; // 测试一个既可以作为名词也可以作为动词的词

        println!("Testing word: {}", word);
        match service.get_word_info(word).await {
            Ok(word_info) => {
                println!("\n=== Word Info Results ===");
                println!("Phonetics:");
                println!("  US: {}", word_info.us_phonetic);
                println!("  UK: {}", word_info.uk_phonetic);
                println!("\nMeanings:");
                for (i, meaning) in word_info.meanings.iter().enumerate() {
                    println!("{}. {} - {}", i + 1, meaning.pos, meaning.definition);
                }

                // Basic validations
                assert!(
                    !word_info.us_phonetic.is_empty(),
                    "US phonetic should not be empty"
                );
                assert!(
                    !word_info.uk_phonetic.is_empty(),
                    "UK phonetic should not be empty"
                );
                assert!(
                    !word_info.meanings.is_empty(),
                    "Should have at least one meaning"
                );

                // Validate meanings
                for meaning in &word_info.meanings {
                    assert!(
                        !meaning.pos.is_empty(),
                        "Part of speech should not be empty"
                    );
                    assert!(
                        !meaning.definition.is_empty(),
                        "Definition should not be empty"
                    );
                    assert!(
                        meaning
                            .definition
                            .chars()
                            .any(|c| (c as u32) > 0x4E00 && (c as u32) < 0x9FFF),
                        "Definition should contain Chinese characters: {}",
                        meaning.definition
                    );
                }

                println!("\n✅ Test passed successfully!");
            }
            Err(e) => {
                println!("❌ Test failed!");
                panic!("Test failed: {}", e);
            }
        }
    }
}
