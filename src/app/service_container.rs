use super::repository_factory::RepositoryFactory;
use crate::app::redis_factory::RedisFactory;
use crate::domain::services::impls::{
    grade_service_impl::GradeServiceImpl, semester_service_impl::SemesterServiceImpl,
    system_config_service_impl::SystemConfigServiceImpl,
    textbook_service_impl::TextbookServiceImpl,
    textbook_version_service_impl::TextbookVersionServiceImpl, unit_service_impl::UnitServiceImpl,
    word_service_impl::WordServiceImpl, word_unit_service_impl::WordUnitServiceImpl,
};
use crate::domain::services::interfaces::grade_service::GradeService;
use crate::domain::services::interfaces::semester_service::SemesterService;
use crate::domain::services::interfaces::textbook_service::TextbookService;
use crate::domain::services::interfaces::unit_service::UnitService;
use crate::domain::services::interfaces::word_service::WordService;
use crate::domain::services::interfaces::word_unit_service::WordUnitService;
use crate::domain::services::interfaces::{SystemConfigService, TextbookVersionService};
use crate::domain::services::{ModelProviderService, ModelProviderServiceImpl};
use crate::infrastructure::third_party::implementations::HongliangServiceImpl;
use crate::infrastructure::third_party::interface::ThirdPartyService;
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::Arc;

pub struct ServiceContainer {
    repository_factory: RepositoryFactory,
    grade_service: OnceCell<Arc<dyn GradeService>>,
    semester_service: OnceCell<Arc<dyn SemesterService>>,
    system_config_service: OnceCell<Arc<dyn SystemConfigService>>,
    textbook_service: OnceCell<Arc<dyn TextbookService>>,
    textbook_version_service: OnceCell<Arc<dyn TextbookVersionService>>,
    unit_service: OnceCell<Arc<dyn UnitService>>,
    word_service: OnceCell<Arc<dyn WordService>>,
    word_unit_service: OnceCell<Arc<dyn WordUnitService>>,
    third_party_service: OnceCell<Arc<dyn ThirdPartyService>>,
    model_provider_service: OnceCell<Arc<dyn ModelProviderService>>,
}

impl ServiceContainer {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            repository_factory: RepositoryFactory::new(db_pool),
            grade_service: OnceCell::new(),
            semester_service: OnceCell::new(),
            system_config_service: OnceCell::new(),
            textbook_service: OnceCell::new(),
            textbook_version_service: OnceCell::new(),
            unit_service: OnceCell::new(),
            word_service: OnceCell::new(),
            word_unit_service: OnceCell::new(),
            third_party_service: OnceCell::new(),
            model_provider_service: OnceCell::new(),
        }
    }

    pub fn get_grade_service(&self) -> Arc<dyn GradeService> {
        self.grade_service
            .get_or_init(|| {
                Arc::new(GradeServiceImpl::new(
                    self.repository_factory.create_grade_repository(),
                ))
            })
            .clone()
    }

    pub fn get_semester_service(&self) -> Arc<dyn SemesterService> {
        self.semester_service
            .get_or_init(|| {
                Arc::new(SemesterServiceImpl::new(
                    self.repository_factory.create_semester_repository(),
                ))
            })
            .clone()
    }

    pub fn get_system_config_service(&self) -> Arc<dyn SystemConfigService> {
        self.system_config_service
            .get_or_init(|| {
                Arc::new(SystemConfigServiceImpl::new(
                    RedisFactory::create_redis_client(),
                ))
            })
            .clone()
    }

    pub fn get_textbook_service(&self) -> Arc<dyn TextbookService> {
        self.textbook_service
            .get_or_init(|| {
                Arc::new(TextbookServiceImpl::new(
                    self.repository_factory.create_textbook_repository(),
                    self.repository_factory.create_textbook_version_repository(),
                    self.repository_factory.create_grade_repository(),
                    self.repository_factory.create_semester_repository(),
                    self.repository_factory.create_unit_repository(),
                ))
            })
            .clone()
    }

    pub fn get_textbook_version_service(&self) -> Arc<dyn TextbookVersionService> {
        self.textbook_version_service
            .get_or_init(|| {
                Arc::new(TextbookVersionServiceImpl::new(
                    self.repository_factory.create_textbook_version_repository(),
                ))
            })
            .clone()
    }

    pub fn get_unit_service(&self) -> Arc<dyn UnitService> {
        self.unit_service
            .get_or_init(|| {
                Arc::new(UnitServiceImpl::new(
                    self.repository_factory.create_unit_repository(),
                ))
            })
            .clone()
    }

    pub fn get_third_party_service(&self) -> Arc<dyn ThirdPartyService> {
        self.third_party_service
            .get_or_init(|| Arc::new(HongliangServiceImpl::new()))
            .clone()
    }

    pub fn get_word_service(&self) -> Arc<dyn WordService> {
        self.word_service
            .get_or_init(|| {
                Arc::new(WordServiceImpl::new(
                    self.repository_factory.create_word_repository(),
                    self.get_system_config_service(),
                    self.get_third_party_service(),
                ))
            })
            .clone()
    }

    pub fn get_word_unit_service(&self) -> Arc<dyn WordUnitService> {
        self.word_unit_service
            .get_or_init(|| {
                Arc::new(WordUnitServiceImpl::new(
                    self.repository_factory
                        .create_word_unit_mapping_repository(),
                    self.repository_factory.create_word_repository(),
                    self.get_word_service(),
                    self.repository_factory.create_unit_repository(),
                    self.repository_factory.create_textbook_repository(),
                ))
            })
            .clone()
    }

    pub fn get_model_provider_service(&self) -> Arc<dyn ModelProviderService> {
        self.model_provider_service
            .get_or_init(|| {
                Arc::new(ModelProviderServiceImpl::new(
                    self.repository_factory.create_model_provider_repository(),
                ))
            })
            .clone()
    }
}
