use crate::api::dto::textbook_dto::TextbookDTO;
use crate::api::dto::unit_dto::UnitDTO;
use crate::domain::models::textbook::Textbook;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

use crate::domain::services::interfaces::textbook_service::TextbookService;
use crate::infrastructure::database::repositories::{
    GradeRepository, SemesterRepository, TextbookRepository, TextbookVersionRepository,
    UnitRepository,
};

pub struct TextbookServiceImpl {
    repository: Arc<dyn TextbookRepository>,
    textbook_version_repository: Arc<dyn TextbookVersionRepository>,
    grade_repository: Arc<dyn GradeRepository>,
    semester_repository: Arc<dyn SemesterRepository>,
    unit_repository: Arc<dyn UnitRepository>,
}

impl TextbookServiceImpl {
    pub fn new(
        repository: Arc<dyn TextbookRepository>,
        textbook_version_repository: Arc<dyn TextbookVersionRepository>,
        grade_repository: Arc<dyn GradeRepository>,
        semester_repository: Arc<dyn SemesterRepository>,
        unit_repository: Arc<dyn UnitRepository>,
    ) -> Self {
        Self {
            repository,
            textbook_version_repository,
            grade_repository,
            semester_repository,
            unit_repository,
        }
    }
}

#[async_trait]
impl TextbookService for TextbookServiceImpl {
    async fn create_textbook(&self, textbook: &mut Textbook) -> Result<Textbook> {
        let version = self
            .textbook_version_repository
            .find_by_id(textbook.id.unwrap())
            .await?
            .unwrap()
            .name;
        let grade = self
            .grade_repository
            .find_by_id(textbook.grade_id.unwrap())
            .await?
            .unwrap()
            .name;
        let semester = self
            .semester_repository
            .find_by_id(textbook.semester_id.unwrap())
            .await?
            .unwrap()
            .name;

        textbook.textbook_version = version;
        textbook.grade = Some(grade);
        textbook.semester = Some(semester);

        self.repository.save(textbook).await
    }

    async fn get_textbooks(&self) -> Result<Vec<TextbookDTO>> {
        let textbooks = self.repository.find_all().await?;
        textbooks
            .into_iter()
            .map(|textbook| TextbookDTO::try_from(textbook).map_err(Into::into))
            .collect::<Result<Vec<_>, _>>()
    }

    async fn delete_textbook(&self, textbook_dto: &TextbookDTO) -> Result<()> {
        self.repository.delete(textbook_dto.id.unwrap()).await
    }

    async fn get_unit_by_textbook(&self, textbook_dto: &TextbookDTO) -> Result<Vec<UnitDTO>> {
        let mut units = vec![];
        if textbook_dto.id.is_some() {
            units = self
                .unit_repository
                .find_by_textbook_id(textbook_dto.id)
                .await?;
        } else {
            //查询textbook id
            let textbook_id = self
                .repository
                .find_by_dto(textbook_dto)
                .await?
                .get(0)
                .unwrap()
                .id;
            units = self
                .unit_repository
                .find_by_textbook_id(textbook_id)
                .await?;
        }
        units
            .into_iter()
            .map(|unit| UnitDTO::try_from(unit).map_err(Into::into))
            .collect::<Result<Vec<_>, _>>()
    }
}
