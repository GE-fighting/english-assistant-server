use crate::domain::models::word::Word;
use crate::domain::services::interfaces::word_service::WordService;
use crate::infrastructure::database::repositories::{Paginated, WordRepository};
use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::services::interfaces::SystemConfigService;
use anyhow::Result;
use crate::infrastructure::llm;
use crate::infrastructure::third_party::ThirdPartyService;

pub struct WordServiceImpl {
    word_repository: Arc<dyn WordRepository>,
    system_config_service: Arc<dyn SystemConfigService>,
    third_party_service: Arc<dyn ThirdPartyService>,
}

impl WordServiceImpl {
    pub fn new(
        word_repository: Arc<dyn WordRepository>,
        system_config_service: Arc<dyn SystemConfigService>,
        third_party_service: Arc<dyn ThirdPartyService>,
    ) -> Self {
        Self {
            word_repository,
            system_config_service,
            third_party_service,
        }
    }

}

#[async_trait]
impl WordService for WordServiceImpl {
    async fn create_word(&self, word: &str) -> Result<Word> {
        //step1. 查询单词是否已存在
        let exist_word = self.word_repository.find_by_word(word).await?;
        if exist_word.is_some() {
            // 判断单词meaning和example是否为空
            let word = exist_word.unwrap();
            if word.meaning.is_some(){
                return Ok(word);
            }
            self.word_repository.delete(word.word_id.unwrap());
        }
        //step2. 构造单词
        let mut word_entity = Word::new(word);
        let pronunciation_us = format!("http://dict.youdao.com/dictvoice?type=0&audio={}", word);
        let pronunciation_uk = format!("http://dict.youdao.com/dictvoice?type=1&audio={}", word);
        word_entity.pronunciation_uk = Some(pronunciation_uk);
        word_entity.pronunciation_us = Some(pronunciation_us);

        let model = self.system_config_service.get_use_model().await?;
        let llm_service = llm::get_llm_manager().get_llm_service(&model)?;
        let example = llm_service.get_example_sentences(word).await?;
        word_entity.example = Some(example);

        if word.contains(" ") {
            let word_info = llm_service.get_word_info(word_entity.word.as_str()).await?;
            word_entity.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));
            word_entity.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
            word_entity.meaning = Some(serde_json::to_string(&word_info.meanings)?)
        }else {
            let word_info = self.third_party_service.fetch_word_info(word).await;
            if word_info.is_ok() {
                let word_info = word_info.unwrap();
                word_entity.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));
                word_entity.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
                word_entity.meaning = Some(serde_json::to_string(&word_info.meanings)?)
            } else {
                let word_info = llm_service.get_word_info(word_entity.word.as_str()).await?;
                word_entity.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));
                word_entity.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
                word_entity.meaning = Some(serde_json::to_string(&word_info.meanings)?)
            }
        }

        //step3. 插入单词
        self.word_repository.save(&word_entity).await
    }

    async fn get_word(&self, word: &str) -> Result<Word> {
        match self.word_repository.find_by_word(word).await? {
            Some(word_entity) => Ok(word_entity),
            None => {
                anyhow::bail!("Word not found: {}", word)
            }
        }
    }

    async fn update_batch_words(&self) -> Result<()> {
        //step1. 查询所有单词
        let mut words: Vec<Word> = self.word_repository.find_all().await?;
        //step2. 获取大模型服务
        let model = self.system_config_service.get_use_model().await?;
        let llm_service = llm::get_llm_manager().get_llm_service(&model)?;
        //step3. 更新所有单词
        for word in &mut words {
            //如果单词没有音标
            if word.meaning.is_none() {
                let word_info = llm_service.get_word_info(word.word.as_str()).await?;
                word.phonetic_uk = Some(word_info.uk_phonetic);
                word.phonetic_us = Some(word_info.us_phonetic);
                word.meaning = Some(serde_json::to_string(&word_info.meanings)?)
            }
            word.pronunciation_us = Some(format!(
                "http://dict.youdao.com/dictvoice?type=0&audio={}",
                word.word
            ));
            word.pronunciation_uk = Some(format!(
                "http://dict.youdao.com/dictvoice?type=1&audio={}",
                word.word
            ));
            //如果单词没有例句和释义
            if word.meaning.is_none() {
                let example = llm_service
                    .get_example_sentences(word.word.as_str())
                    .await?;
                word.example = Some(example);
            }
            self.word_repository.save(&word).await?;
        }
        Ok(())
    }

}
