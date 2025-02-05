use crate::infrastructure::database::repositories::{
    GradeRepository, GradeRepositoryImpl, ModelProviderRepository, ModelProviderRepositoryImpl,
    SemesterRepository, SemesterRepositoryImpl, TextbookRepository, TextbookRepositoryImpl,
    TextbookVersionRepository, TextbookVersionRepositoryImpl, UnitRepository, UnitRepositoryImpl,
    WordRepository, WordRepositoryImpl, WordUnitMappingRepository, WordUnitMappingRepositoryImpl,
};
use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::sync::Arc;

pub struct RepositoryFactory {
    db_pool: Arc<PgPool>,
    grade_repository: OnceCell<Arc<dyn GradeRepository>>,
    semester_repository: OnceCell<Arc<dyn SemesterRepository>>,
    textbook_repository: OnceCell<Arc<dyn TextbookRepository>>,
    textbook_version_repository: OnceCell<Arc<dyn TextbookVersionRepository>>,
    unit_repository: OnceCell<Arc<dyn UnitRepository>>,
    word_repository: OnceCell<Arc<dyn WordRepository>>,
    word_unit_mapping_repository: OnceCell<Arc<dyn WordUnitMappingRepository>>,
    model_provider_repository: OnceCell<Arc<dyn ModelProviderRepository>>,
}

impl RepositoryFactory {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self {
            db_pool,
            grade_repository: OnceCell::new(),
            semester_repository: OnceCell::new(),
            textbook_repository: OnceCell::new(),
            textbook_version_repository: OnceCell::new(),
            unit_repository: OnceCell::new(),
            word_repository: OnceCell::new(),
            word_unit_mapping_repository: OnceCell::new(),
            model_provider_repository: OnceCell::new(),
        }
    }

    pub fn create_grade_repository(&self) -> Arc<dyn GradeRepository> {
        self.grade_repository
            .get_or_init(|| Arc::new(GradeRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_semester_repository(&self) -> Arc<dyn SemesterRepository> {
        self.semester_repository
            .get_or_init(|| Arc::new(SemesterRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_textbook_repository(&self) -> Arc<dyn TextbookRepository> {
        self.textbook_repository
            .get_or_init(|| Arc::new(TextbookRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_textbook_version_repository(&self) -> Arc<dyn TextbookVersionRepository> {
        self.textbook_version_repository
            .get_or_init(|| Arc::new(TextbookVersionRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_unit_repository(&self) -> Arc<dyn UnitRepository> {
        self.unit_repository
            .get_or_init(|| Arc::new(UnitRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_word_repository(&self) -> Arc<dyn WordRepository> {
        self.word_repository
            .get_or_init(|| Arc::new(WordRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_word_unit_mapping_repository(&self) -> Arc<dyn WordUnitMappingRepository> {
        self.word_unit_mapping_repository
            .get_or_init(|| Arc::new(WordUnitMappingRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }

    pub fn create_model_provider_repository(&self) -> Arc<dyn ModelProviderRepository> {
        self.model_provider_repository
            .get_or_init(|| Arc::new(ModelProviderRepositoryImpl::new(self.db_pool.clone())))
            .clone()
    }
}
