mod base;
mod grade_repository;
pub(crate) mod model_provider_repository;
mod semester_repository;
mod textbook_repository;
mod textbook_version_repository;
mod unit_repository;
mod word_repository;
mod word_unit_mapping_repository;

pub use base::{Paginated, Repository};
pub use grade_repository::{GradeRepository, GradeRepositoryImpl};
pub use model_provider_repository::{ModelProviderRepository, ModelProviderRepositoryImpl};
pub use semester_repository::{SemesterRepository, SemesterRepositoryImpl};
pub use textbook_repository::{TextbookRepository, TextbookRepositoryImpl};
pub use textbook_version_repository::{TextbookVersionRepository, TextbookVersionRepositoryImpl};
pub use unit_repository::{UnitRepository, UnitRepositoryImpl};
pub use word_repository::{WordRepository, WordRepositoryImpl};
pub use word_unit_mapping_repository::{WordUnitMappingRepository, WordUnitMappingRepositoryImpl};
