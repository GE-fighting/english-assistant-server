use sqlx::PgPool;
use std::sync::Arc;

use crate::domain::services::interfaces::grade_service::GradeService;
use crate::domain::services::interfaces::semester_service::SemesterService;
use crate::domain::services::interfaces::textbook_service::TextbookService;
use crate::domain::services::interfaces::unit_service::UnitService;
use crate::domain::services::interfaces::word_service::WordService;
use crate::domain::services::interfaces::word_unit_service::WordUnitService;
use crate::domain::services::interfaces::TextbookVersionService;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>,
    pub grade_service: Arc<dyn GradeService>,
    pub semester_service: Arc<dyn SemesterService>,
    pub textbook_service: Arc<dyn TextbookService>,
    pub textbook_version_service: Arc<dyn TextbookVersionService>,
    pub unit_service: Arc<dyn UnitService>,
    pub word_service: Arc<dyn WordService>,
    pub word_unit_service: Arc<dyn WordUnitService>,
}

impl AppState {
    pub fn new(
        db_pool: Arc<PgPool>,
        grade_service: Arc<dyn GradeService>,
        semester_service: Arc<dyn SemesterService>,
        textbook_service: Arc<dyn TextbookService>,
        textbook_version_service: Arc<dyn TextbookVersionService>,
        unit_service: Arc<dyn UnitService>,
        word_service: Arc<dyn WordService>,
        word_unit_service: Arc<dyn WordUnitService>,
    ) -> Self {
        Self {
            db_pool,
            grade_service,
            semester_service,
            textbook_service,
            textbook_version_service,
            unit_service,
            word_service,
            word_unit_service,
        }
    }
}
