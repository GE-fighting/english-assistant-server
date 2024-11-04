use actix_web::web;
use crate::handlers::{textbook_version_handler, word_handler, grade_handler};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/textbook")
                    .route("/versions", web::get().to(textbook_version_handler::get_textbook_versions))
            )
            .service(
                web::scope("/word")
                    .route("/create", web::post().to(word_handler::create_word))
                    .route("/get", web::post().to(word_handler::get_word))
                    .route("/update", web::post().to(word_handler::update_batch_words))
            )
            .service(
                web::scope("/grade")
                    .route("/list", web::get().to(grade_handler::get_grades))
            )
    );
}
