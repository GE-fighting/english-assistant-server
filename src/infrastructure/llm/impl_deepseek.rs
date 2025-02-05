use crate::infrastructure::dto::WordInfo;
use crate::infrastructure::llm::interface::LLMService;
use crate::infrastructure::llm::prompts::LanguagePrompts;
use crate::infrastructure::llm::provider::LLMConfig;
use crate::infrastructure::llm::utils;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use deepseek_api_client::Message;
use dotenv::dotenv;
use std::env;
use tokio::runtime::Handle;
use tracing::debug;

pub struct DeepSeekServiceImpl {
    api_key: Option<String>,
    timeout: u64,
}

impl DeepSeekServiceImpl {
    pub fn new() -> Result<Self> {
        Ok(Self {
            api_key: None,
            timeout: 30,
        })
    }

    /// Helper function to execute a chat completion request
    async fn execute_chat(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        temperature: f32,
    ) -> Result<String> {
        let messages = vec![
            Message {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            Message {
                role: "user".to_string(),
                content: user_prompt.to_string(),
            },
        ];
        if self.api_key.is_none() {
            return Err(anyhow!("No API key provided"));
        }
        let api_key = self.api_key.clone().unwrap();

        // Run the blocking operation in a separate thread
        let handle = Handle::current();
        let response = handle
            .spawn_blocking(move || {
                let mut chat_fn = if temperature > 0.5 {
                    deepseek_api_client::chat_completion_sync(&api_key)
                } else {
                    deepseek_api_client::code_completion_sync(&api_key)
                };
                chat_fn(messages)
            })
            .await??;

        deepseek_api_client::get_response_text(&response, 0)
            .ok_or_else(|| anyhow!("No response received from DeepSeek API"))
    }
}

#[async_trait]
impl LLMService for DeepSeekServiceImpl {
    fn configure(&mut self, config: &LLMConfig) -> Result<()> {
        self.api_key = config.api_key.clone();
        self.timeout = config.timeout.unwrap();
        Ok(())
    }

    async fn get_phonetics(&self, word: &str) -> Result<(String, String)> {
        let content = self
            .execute_chat(
                LanguagePrompts::PHONETICS_SYSTEM,
                &LanguagePrompts::phonetics_user(word),
                0.3,
            )
            .await?;
        debug!(" get phonetics from AI response: {:?}", content);
        let cleaned_content = utils::clean_json_response(&content);
        utils::extract_phonetics(cleaned_content.as_str())
    }

    async fn get_example_sentences(&self, word: &str) -> Result<String> {
        let content = self
            .execute_chat(
                LanguagePrompts::SENTENCES_SYSTEM,
                &LanguagePrompts::example_sentences_user(word),
                0.7,
            )
            .await?;
        debug!("example sentences from AI response : {}", content);
        let cleaned_content = utils::clean_json_response(&content);
        utils::extract_example_sentences(cleaned_content.as_str())
    }

    async fn get_word_info(&self, word: &str) -> Result<WordInfo> {
        let content = self
            .execute_chat(
                LanguagePrompts::WORD_INFO_SYSTEM,
                &LanguagePrompts::word_info_user(word),
                0.3,
            )
            .await?;

        // Clean the response before parsing
        let cleaned_content = utils::clean_json_response(&content);
        debug!("get word info from AI response : {}", content);

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

    fn setup() {
        // Ensure .env is loaded before each test
        dotenv().ok();
        // Print current environment variable to help debug
        if let Ok(api_key) = env::var("LLM_DEEPSEEK_API_KEY") {
            println!(
                "Found API key: {}...{}",
                &api_key[..8],
                &api_key[api_key.len() - 4..]
            );
        } else {
            println!("Warning: LLM_DEEP_SEEK_API_KEY not found in environment");
        }
    }

    #[tokio::test]
    async fn test_get_phonetics() {
        println!("\n=== Testing get_phonetics ===");
        setup(); // Ensure environment is properly set up
        match DeepSeekServiceImpl::new() {
            Ok(service) => {
                println!("Service initialized successfully");
                let word = "hello";
                println!("Testing word: {}", word);

                match service.get_phonetics(word).await {
                    Ok((us, uk)) => {
                        println!("✅ Test successful!");
                        println!("Results for word '{}':", word);
                        println!("US phonetic: {}", us);
                        println!("UK phonetic: {}", uk);

                        assert!(!us.is_empty(), "US phonetic is empty");
                        assert!(!uk.is_empty(), "UK phonetic is empty");
                        assert!(
                            us.contains('/') || us.contains('['),
                            "US phonetic lacks proper formatting: {}",
                            us
                        );
                        assert!(
                            uk.contains('/') || uk.contains('['),
                            "UK phonetic lacks proper formatting: {}",
                            uk
                        );
                    }
                    Err(e) => {
                        println!("❌ Test failed!");
                        println!("Error: {}", e);
                        panic!("Test failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to initialize service");
                println!("Error: {}", e);
                panic!("Failed to initialize service: {}", e);
            }
        }
        println!("=== Test completed ===\n");
    }

    #[tokio::test]
    async fn test_get_example_sentences() {
        println!("\n=== Testing get_example_sentences ===");
        setup(); // Ensure environment is properly set up
        match DeepSeekServiceImpl::new() {
            Ok(service) => {
                println!("Service initialized successfully");
                let word = "hello";
                println!("Testing word: {}", word);

                match service.get_example_sentences(word).await {
                    Ok(sentences) => {
                        println!("✅ Test successful!");
                        println!("Results for word '{}':", word);

                        assert_eq!(
                            sentences.len(),
                            2,
                            "Expected 2 sentences, got {}",
                            sentences.len()
                        );

                        println!("\nSimple sentence:");
                        println!("English: {}", sentences[0].0);
                        println!("Chinese: {}", sentences[0].1);
                        println!("\nComplex sentence:");
                        println!("English: {}", sentences[1].0);
                        println!("Chinese: {}", sentences[1].1);

                        for (i, (en, zh)) in sentences.iter().enumerate() {
                            // Validate English sentence
                            assert!(!en.is_empty(), "English sentence {} is empty", i + 1);
                            assert!(
                                en.to_lowercase().contains(word),
                                "Sentence {} doesn't contain the word '{}': {}",
                                i + 1,
                                word,
                                en
                            );
                            println!("\nValidation for sentence {}:", i + 1);
                            println!("✓ English sentence is not empty");
                            println!("✓ Contains target word '{}'", word);

                            // Validate Chinese translation
                            assert!(!zh.is_empty(), "Chinese translation {} is empty", i + 1);
                            assert!(
                                zh.chars()
                                    .any(|c| (c as u32) > 0x4E00 && (c as u32) < 0x9FFF),
                                "Translation {} should contain Chinese characters: {}",
                                i + 1,
                                zh
                            );
                            println!("✓ Chinese translation is not empty");
                            println!("✓ Contains Chinese characters");
                        }
                    }
                    Err(e) => {
                        println!("❌ Test failed!");
                        println!("Error: {}", e);
                        panic!("Test failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to initialize service");
                println!("Error: {}", e);
                panic!("Failed to initialize service: {}", e);
            }
        }
        println!("=== Test completed ===\n");
    }

    #[tokio::test]
    async fn test_get_word_info() {
        println!("\n=== Testing get_word_info ===");
        setup(); // Ensure environment is properly set up

        match DeepSeekServiceImpl::new() {
            Ok(service) => {
                println!("Service initialized successfully");
                let word = "name";
                println!("Testing word: {}", word);

                match service.get_word_info(word).await {
                    Ok(word_info) => {
                        println!("✅ Test successful!");
                        println!("Results for word '{}':", word);
                        println!("US phonetic: {}", word_info.us_phonetic);
                        println!("UK phonetic: {}", word_info.uk_phonetic);
                        println!("\nMeanings:");
                        for (i, meaning) in word_info.meanings.iter().enumerate() {
                            println!("{}. {} - {}", i + 1, meaning.pos, meaning.definition);
                        }

                        // Validate response
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
                            "Meanings should not be empty"
                        );

                        for meaning in &word_info.meanings {
                            assert!(
                                !meaning.pos.is_empty(),
                                "Part of speech should not be empty"
                            );
                            assert!(
                                !meaning.definition.is_empty(),
                                "Definition should not be empty"
                            );
                        }
                    }
                    Err(e) => {
                        println!("❌ Test failed!");
                        println!("Error: {}", e);
                        panic!("Test failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to initialize service");
                println!("Error: {}", e);
                panic!("Failed to initialize service: {}", e);
            }
        }
        println!("=== Test completed ===\n");
    }
}
