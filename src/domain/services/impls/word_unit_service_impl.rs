use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::api::dto::unit_word_dto::WordDTO;
use crate::domain::models::word_unit_mapping::WordUnitMapping;
use crate::domain::services::interfaces::word_service::WordService;

use crate::domain::services::interfaces::word_unit_service::WordUnitService;
use crate::infrastructure::database::repositories::{
    TextbookRepository, UnitRepository, WordRepository, WordUnitMappingRepository,
};

pub struct WordUnitServiceImpl {
    word_unit_repository: Arc<dyn WordUnitMappingRepository>,
    word_repository: Arc<dyn WordRepository>,
    word_service: Arc<dyn WordService>,
    unit_repository: Arc<dyn UnitRepository>,
    textbook_repository: Arc<dyn TextbookRepository>,
}

impl WordUnitServiceImpl {
    pub fn new(
        word_unit_repository: Arc<dyn WordUnitMappingRepository>,
        word_repository: Arc<dyn WordRepository>,
        word_service: Arc<dyn WordService>,
        unit_repository: Arc<dyn UnitRepository>,
        textbook_repository: Arc<dyn TextbookRepository>,
    ) -> Self {
        Self {
            word_unit_repository,
            word_repository,
            word_service,
            unit_repository,
            textbook_repository,
        }
    }
}

#[async_trait]
impl WordUnitService for WordUnitServiceImpl {
    async fn get_unit_words(&self, unit_id: i32) -> Result<Vec<WordDTO>> {
        self.word_unit_repository
            .find_word_dto_by_unit_id(unit_id)
            .await
    }

    async fn create_word_unit_mapping(&self, unit_word_dto: &WordDTO) -> Result<WordDTO> {
        //step1. 拿到单词信息
        let word = unit_word_dto.word.clone().unwrap();
        let word_entity = self.word_service.create_word(word.as_str()).await?;

        //step2. 绑定单元
        let mut unit_word = WordUnitMapping::new();
        unit_word.word_id = Some(word_entity.word_id.unwrap());
        unit_word.unit_id = Some(unit_word_dto.unit_id.unwrap());
        let unit_word = self.word_unit_repository.save(&unit_word).await?;
        //step3. 更新单元单词数
        let mut unit = self
            .unit_repository
            .find_by_id(unit_word_dto.unit_id.unwrap())
            .await?
            .expect("unit not found");
        unit.word_count = Some(unit.word_count.unwrap() + 1);
        self.unit_repository.save(&unit).await?;
        //step4. 更新课本单词数
        let unit_id = self
            .unit_repository
            .find_by_id(unit_word_dto.unit_id.unwrap())
            .await?
            .unwrap()
            .textbook_id
            .unwrap();
        let mut textbook = self.textbook_repository.find_by_id(unit_id).await?.unwrap();
        textbook.word_count = Some(textbook.word_count.unwrap() + 1);
        self.textbook_repository.save(&textbook).await?;

        Ok(WordDTO::new(&word_entity, &unit_word))
    }

    async fn delete_unit_word(&self, id: i32) -> Result<()> {
        let unit_id = self
            .word_unit_repository
            .find_by_id(id)
            .await?
            .expect("unit not found")
            .unit_id
            .expect("unit not found");
        let mut unit = self
            .unit_repository
            .find_by_id(unit_id)
            .await?
            .expect("unit not found");
        unit.word_count = Some(unit.word_count.unwrap() - 1);
        self.unit_repository.save(&unit).await?;
        self.word_unit_repository.delete(id).await
    }
}
