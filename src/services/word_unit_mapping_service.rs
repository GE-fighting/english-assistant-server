use crate::models::dto::unit_word_dto::UnitWordDTO;
use crate::models::response::ApiResponse;
use crate::models::word::Word;
use crate::repositories::word_unit_mapping_repository::find_by_unit_id;
use crate::repositories::{unit_repository, word_repository, word_unit_mapping_repository};
use crate::services::external_service::hongliang_service::fetch_word_info;
use crate::services::external_service::llm_service::LLMService;
use anyhow::{Error, Ok, Result};
use sqlx::PgPool;

pub async fn get_unit_words(pool: &PgPool, unit_id: i32) -> ApiResponse<Vec<UnitWordDTO>> {
    let words = find_by_unit_id(pool, unit_id).await.unwrap();
    //用流处理
    let mut unit_words_dto = vec![];
    for word in words.iter() {
        let word_record = word_repository::find_by_id(pool, word.word_id)
            .await
            .unwrap();
        if let Some(record) = word_record {
            unit_words_dto.push(UnitWordDTO::new(&record, word));
        }
    }
    ApiResponse::success(unit_words_dto)
}

pub async fn create_word_unit_mapping(
    pool: &PgPool,
    unit_word_dto: &UnitWordDTO,
) -> Result<UnitWordDTO> {
    //1、查看单词是否存在，不存在新建
    let word = unit_word_dto.word.clone().unwrap();
    let word_record = word_repository::find_by_word(pool, &word).await.unwrap();
    let word_entity = match word_record {
        Some(mut record) => {
            if record.meaning.is_some() {
                record
            } else {
                //更新单词
                let word_info = fetch_word_info(&*word).await.unwrap();

                record.meaning = Some(serde_json::to_string(&word_info.meanings).unwrap());
                record.phonetic_us = Some(format!("/{}/", word_info.us_phonetic));
                record.phonetic_uk = Some(format!("/{}/", word_info.uk_phonetic));
                let llm_service = LLMService::new();
                let sentences = llm_service.get_example_sentences(&word).await?;
                let mut examples = String::new();
                for (eng, chn) in sentences {
                    examples.push_str(&format!("{}|{}\n", eng, chn));
                }
                record.example = Some(examples);

                word_repository::update(pool, &record).await.unwrap()
            }
        }
        None => {
            let mut word_entity = Word::new(&word);
            let word_info = fetch_word_info(&*word).await.unwrap();

            word_entity.meaning = Some(serde_json::to_string(&word_info.meanings).unwrap());
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
            let llm_service = LLMService::new();
            let sentences = llm_service.get_example_sentences(&word).await?;
            let mut examples = String::new();
            for (eng, chn) in sentences {
                examples.push_str(&format!("{}|{}\n", eng, chn));
            }
            word_entity.example = Some(examples);
            word_repository::create(pool, &word_entity).await.unwrap()
        }
    };
    //2、绑定单词和单元
    let create_result = word_unit_mapping_repository::create(
        pool,
        word_entity.word_id.unwrap(),
        unit_word_dto.unit_id.unwrap(),
    )
    .await?;
    //更新单元的单词数量
    let mut unit_entity = unit_repository::find_by_id(pool, unit_word_dto.unit_id.unwrap()).await?;
    unit_entity.word_count = Option::from(unit_entity.word_count.unwrap() + 1);
    let updated_unit = unit_repository::update_unit(pool, &unit_entity).await?;
    Ok(UnitWordDTO::new(&word_entity, &create_result))
}

//删除单元单词记录
pub async fn delete_unit_word(pool: &PgPool, id: i32) -> Result<()> {
    word_unit_mapping_repository::delete_by_id(pool, id).await
}
