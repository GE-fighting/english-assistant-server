pub mod route_macros;

use crate::api::handler::{
    grade_handler::GradeHandler, semester_handler::SemesterHandler,
    system_config_handler::SystemConfigHandler, textbook_handler::TextbookHandler,
    textbook_version_handler::TextbookVersionHandler, unit_handler::UnitHandler,
    word_handler::WordHandler, word_unit_handler::WordUnitHandler, Handler,
};
use crate::app::HandlerFactory;
use actix_web::web;
use crate::api::handler::model_provider_handler::ModelProviderHandler;
use crate::domain::models::model_provider::ModelProvider;

/// Configure all application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig, handler_factory: HandlerFactory) {
    // Create handlers once
    let grade_handler = web::Data::new(handler_factory.create_grade_handler());
    let semester_handler = web::Data::new(handler_factory.create_semester_handler());
    let textbook_handler = web::Data::new(handler_factory.create_textbook_handler());
    let textbook_version_handler =
        web::Data::new(handler_factory.create_textbook_version_handler());
    let unit_handler = web::Data::new(handler_factory.create_unit_handler());
    let word_handler = web::Data::new(handler_factory.create_word_handler());
    let word_unit_handler = web::Data::new(handler_factory.create_word_unit_handler());
    let system_config_handler = web::Data::new(handler_factory.create_system_config_handler());
    let model_provider = web::Data::new(handler_factory.create_model_provider_handler());

    cfg.service(
        web::scope("/api")
            .app_data(grade_handler.clone())
            .app_data(semester_handler.clone())
            .app_data(textbook_handler.clone())
            .app_data(textbook_version_handler.clone())
            .app_data(unit_handler.clone())
            .app_data(word_handler.clone())
            .app_data(word_unit_handler.clone())
            .app_data(system_config_handler.clone())
            .app_data(model_provider.clone())
            .service(web::scope("/grade").configure(GradeHandler::register))
            .service(web::scope("/semester").configure(SemesterHandler::register))
            .service(web::scope("/textbook").configure(TextbookHandler::register))
            .service(web::scope("/textbook-version").configure(TextbookVersionHandler::register))
            .service(web::scope("/unit").configure(UnitHandler::register))
            .service(web::scope("/word").configure(WordHandler::register))
            .service(web::scope("/unit-word").configure(WordUnitHandler::register))
            .service(web::scope("/system").configure(SystemConfigHandler::register))
            .service(web::scope("/model").configure(ModelProviderHandler::register)),

    );
}
