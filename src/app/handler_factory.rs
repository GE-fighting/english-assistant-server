use crate::api::handler::grade_handler::GradeHandler;
use crate::api::handler::model_provider_handler::ModelProviderHandler;
use crate::api::handler::semester_handler::SemesterHandler;
use crate::api::handler::system_config_handler::SystemConfigHandler;
use crate::api::handler::textbook_handler::TextbookHandler;
use crate::api::handler::textbook_version_handler::TextbookVersionHandler;
use crate::api::handler::unit_handler::UnitHandler;
use crate::api::handler::word_handler::WordHandler;
use crate::api::handler::word_unit_handler::WordUnitHandler;
use crate::domain::services::interfaces::grade_service::GradeService;
use crate::domain::services::interfaces::semester_service::SemesterService;
use crate::domain::services::interfaces::textbook_service::TextbookService;
use crate::domain::services::interfaces::unit_service::UnitService;
use crate::domain::services::interfaces::word_service::WordService;
use crate::domain::services::interfaces::word_unit_service::WordUnitService;
use crate::domain::services::interfaces::{SystemConfigService, TextbookVersionService};
use crate::domain::services::ModelProviderService;
use std::sync::Arc;

#[derive(Clone)]
pub struct HandlerFactory {
    grade_service: Arc<dyn GradeService>,
    semester_service: Arc<dyn SemesterService>,
    system_config_service: Arc<dyn SystemConfigService>,
    textbook_service: Arc<dyn TextbookService>,
    textbook_version_service: Arc<dyn TextbookVersionService>,
    unit_service: Arc<dyn UnitService>,
    word_service: Arc<dyn WordService>,
    word_unit_service: Arc<dyn WordUnitService>,
    model_provider_service: Arc<dyn ModelProviderService>,
}

impl HandlerFactory {
    pub fn new(
        grade_service: Arc<dyn GradeService>,
        semester_service: Arc<dyn SemesterService>,
        system_config_service: Arc<dyn SystemConfigService>,
        textbook_service: Arc<dyn TextbookService>,
        textbook_version_service: Arc<dyn TextbookVersionService>,
        unit_service: Arc<dyn UnitService>,
        word_service: Arc<dyn WordService>,
        word_unit_service: Arc<dyn WordUnitService>,
        model_provider_service: Arc<dyn ModelProviderService>,
    ) -> Self {
        Self {
            grade_service,
            semester_service,
            system_config_service,
            textbook_service,
            textbook_version_service,
            unit_service,
            word_service,
            word_unit_service,
            model_provider_service,
        }
    }

    pub fn create_grade_handler(&self) -> GradeHandler {
        GradeHandler::new(self.grade_service.clone())
    }

    pub fn create_semester_handler(&self) -> SemesterHandler {
        SemesterHandler::new(self.semester_service.clone())
    }

    pub fn create_system_config_handler(&self) -> SystemConfigHandler {
        SystemConfigHandler::new(self.system_config_service.clone())
    }

    pub fn create_textbook_handler(&self) -> TextbookHandler {
        TextbookHandler::new(self.textbook_service.clone())
    }

    pub fn create_textbook_version_handler(&self) -> TextbookVersionHandler {
        TextbookVersionHandler::new(self.textbook_version_service.clone())
    }

    pub fn create_unit_handler(&self) -> UnitHandler {
        UnitHandler::new(self.unit_service.clone())
    }

    pub fn create_word_handler(&self) -> WordHandler {
        WordHandler::new(self.word_service.clone())
    }

    pub fn create_word_unit_handler(&self) -> WordUnitHandler {
        WordUnitHandler::new(self.word_unit_service.clone())
    }

    pub fn create_model_provider_handler(&self) -> ModelProviderHandler {
        ModelProviderHandler::new(self.model_provider_service.clone())
    }
}
