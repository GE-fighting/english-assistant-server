use crate::api::dto::textbook_dto::TextbookDTO;
use crate::api::dto::unit_dto::UnitDTO;
use crate::domain::models::textbook::Textbook;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TextbookService: Send + Sync {
    async fn create_textbook(&self, textbook: &mut Textbook) -> Result<Textbook>;
    async fn get_textbooks(&self) -> Result<Vec<TextbookDTO>>;
    async fn delete_textbook(&self, textbook_dto: &TextbookDTO) -> Result<()>;
    async fn get_unit_by_textbook(&self, textbook_dto: &TextbookDTO) -> Result<Vec<UnitDTO>>;
}
