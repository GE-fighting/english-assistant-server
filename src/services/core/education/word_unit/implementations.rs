use anyhow::Result;
use async_trait::async_trait;
use sqlx::PgPool;

use crate::models::dto::unit_word_dto::UnitWordDTO;
use crate::models::response::ApiResponse;
use crate::models::word::Word;
use crate::repositories::{unit_repository, word_repository, word_unit_mapping_repository};
use crate::services::external::llm::{LLMService, LLMServiceImpl};
use crate::services::external::third_party::{HongliangServiceImpl, ThirdPartyService};

use super::interface::WordUnitService;

pub struct WordUnitServiceImpl {
    pool: PgPool,
}

impl WordUnitServiceImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WordUnitService for WordUnitServiceImpl {
    async fn get_unit_words(&self, unit_id: i32) -> ApiResponse<Vec<UnitWordDTO>> {
        let words = word_unit_mapping_repository::find_by_unit_id(&self.pool, unit_id)
            .await
            .unwrap();

        let mut unit_words_dto = vec![];
        for word in words.iter() {
            let word_record = word_repository::find_by_id(&self.pool, word.word_id)
                .await
                .unwrap();
            if let Some(record) = word_record {
                unit_words_dto.push(UnitWordDTO::new(&record, word));
            }
        }
        ApiResponse::success(unit_words_dto)
    }

    async fn create_word_unit_mapping(&self, unit_word_dto: &UnitWordDTO) -> Result<UnitWordDTO> {
        let word = unit_word_dto.word.clone().unwrap();
        let word_record = word_repository::find_by_word(&self.pool, &word)
            .await
            .unwrap();

        let word_entity = match word_record {
            Some(mut record) => {
                if record.meaning.is_some() {
                    record
                } else {
                    let hongliang_service = HongliangServiceImpl::new();
                    let word_info = hongliang_service.fetch_word_info(&*word).await?;

                    record.meaning = Some(serde_json::to_string(&word_info.meanings)?);
                    record.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
                    record.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));

                    let llm_service = LLMServiceImpl::new();
                    let sentences = llm_service.get_example_sentences(&word).await?;
                    let mut examples = String::new();
                    for (eng, chn) in sentences {
                        examples.push_str(&format!("{}|{}\n", eng, chn));
                    }
                    record.example = Some(examples);

                    word_repository::update(&self.pool, &record).await?
                }
            }
            None => {
                let mut word_entity = Word::new(&word);
                let hongliang_service = HongliangServiceImpl::new();
                let word_info = hongliang_service.fetch_word_info(&*word).await?;

                word_entity.meaning = Some(serde_json::to_string(&word_info.meanings)?);
                word_entity.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
                word_entity.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));
                word_entity.pronunciation_uk = Some(format!(
                    "http://dict.youdao.com/dictvoice?type=0&audio={}",
                    word
                ));
                word_entity.pronunciation_us = Some(format!(
                    "http://dict.youdao.com/dictvoice?type=1&audio={}",
                    word
                ));

                let llm_service = LLMServiceImpl::new();
                let sentences = llm_service.get_example_sentences(&word).await?;
                let mut examples = String::new();
                for (eng, chn) in sentences {
                    examples.push_str(&format!("{}|{}\n", eng, chn));
                }
                word_entity.example = Some(examples);
                word_repository::create(&self.pool, &word_entity).await?
            }
        };

        let create_result = word_unit_mapping_repository::create(
            &self.pool,
            word_entity.word_id.unwrap(),
            unit_word_dto.unit_id.unwrap(),
        )
        .await?;

        let mut unit_entity =
            unit_repository::find_by_id(&self.pool, unit_word_dto.unit_id.unwrap()).await?;
        unit_entity.word_count = Some(unit_entity.word_count.unwrap() + 1);
        unit_repository::update_unit(&self.pool, &unit_entity).await?;

        Ok(UnitWordDTO::new(&word_entity, &create_result))
    }

    async fn delete_unit_word(&self, id: i32) -> Result<()> {
        word_unit_mapping_repository::delete_by_id(&self.pool, id).await
    }
}
