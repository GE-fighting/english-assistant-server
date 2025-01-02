use anyhow::{anyhow, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};

use super::interface::LLMService;

pub struct LLMServiceImpl {
    client: Client,
    api_key: String,
    api_url: String,
}

impl LLMServiceImpl {
    pub fn new() -> Self {
        use crate::config;
        let key = config::Settings::global()
            .get_lingyiwanwu_api_key()
            .to_string();
        Self {
            client: Client::new(),
            api_key: key,
            api_url: "https://api.lingyiwanwu.com/v1/chat/completions".to_string(),
        }
    }
}

#[async_trait]
impl LLMService for LLMServiceImpl {
    async fn get_phonetics(&self, word: &str) -> Result<(String, String)> {
        let prompt = format!(
            "Please provide the International Phonetic Alphabet (IPA) pronunciations for the English word '{}'. \
            Return the American (US) IPA and British (UK) IPA, \
            separated by a comma in that order. Do not include any additional text or explanation.",
            word
        );

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(json!({
                "model": "yi-lightning",
                "messages": [
                    {"role": "system", "content": "You are a linguistic expert specializing in English phonetics."},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.3
            }).to_string())
            .send()
            .await?;

        let response_body: Value = serde_json::from_str(&response.text().await?)?;
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to extract content from AI response"))?;

        let mut content_iter = content.split(',');
        let us_phonetic = content_iter
            .next()
            .ok_or_else(|| anyhow!("Failed to extract US phonetic"))?
            .trim()
            .to_string();

        let uk_phonetic = content_iter
            .next()
            .ok_or_else(|| anyhow!("Failed to extract UK phonetic"))?
            .trim()
            .to_string();

        Ok((us_phonetic, uk_phonetic))
    }

    async fn get_example_sentences(&self, word: &str) -> Result<Vec<(String, String)>> {
        let prompt = format!(
            "Please provide two example sentences using the word '{}'. \
            One sentence should be simple, and the other should be more complex. \
            For each sentence, provide its Chinese translation. \
            Format: English sentence 1|Chinese translation 1||English sentence 2|Chinese translation 2",
            word
        );

        let response = self.client
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .body(json!({
                "model": "yi-lightning",
                "messages": [
                    {"role": "system", "content": "You are a language expert specializing in creating example sentences."},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.7
            }).to_string())
            .send()
            .await?;

        let response_body: Value = serde_json::from_str(&response.text().await?)?;
        let content = response_body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Failed to extract content from AI response"))?
            .trim()
            .to_string();

        let mut result = Vec::new();
        for sentence_pair in content.split("||") {
            let mut parts = sentence_pair.split('|');
            let english = parts
                .next()
                .ok_or_else(|| anyhow!("Failed to extract English sentence from AI response"))?
                .trim()
                .to_string();
            let chinese = parts
                .next()
                .ok_or_else(|| anyhow!("Failed to extract Chinese translation from AI response"))?
                .trim()
                .to_string();
            result.push((english, chinese));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_example_sentences() {
        let service = LLMServiceImpl::new();
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
        let service = LLMServiceImpl::new();
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
}
