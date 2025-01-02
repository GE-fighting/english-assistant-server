use anyhow::Result;
use async_trait::async_trait;

use crate::models::dto::textbook_dto::TextbookDTO;
use crate::models::dto::unit_dto::UnitDTO;
use crate::models::response::ApiResponse;
use crate::models::textbook::Textbook;

#[async_trait]
pub trait TextbookService {
    async fn create_textbook(&self, textbook: &Textbook) -> ApiResponse<Textbook>;
    async fn get_textbooks(&self) -> ApiResponse<Vec<TextbookDTO>>;
    async fn delete_textbook(&self, textbook_dto: &TextbookDTO) -> Result<()>;
    async fn get_unit_by_textbook(&self, textbook_dto: &TextbookDTO) -> Result<Vec<UnitDTO>>;
}
