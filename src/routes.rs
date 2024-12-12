use crate::handlers::{
    grade_handler, semester_handler, textbook_handler, textbook_version_handler, unit_handler,
    word_handler, word_unit_mapping_handler,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/textbook-version")
                    .route(
                        "/list",
                        web::get().to(textbook_version_handler::get_textbook_versions),
                    )
                    .route(
                        "/create",
                        web::post().to(textbook_version_handler::create_textbook_version),
                    )
                    .route(
                        "/update",
                        web::post().to(textbook_version_handler::update_textbook_version),
                    )
                    .route(
                        "/delete",
                        web::post().to(textbook_version_handler::delete_textbook_version),
                    ),
            )
            .service(
                web::scope("/word")
                    .route("/create", web::post().to(word_handler::create_word))
                    .route("/get", web::post().to(word_handler::get_word))
                    .route("/update", web::post().to(word_handler::update_batch_words)),
            )
            .service(web::scope("/grade").route("/list", web::get().to(grade_handler::get_grades)))
            .service(
                web::scope("/semester")
                    .route("/list", web::get().to(semester_handler::get_semester)),
            )
            .service(
                web::scope("/textbook")
                    .route("/list", web::get().to(textbook_handler::get_textbooks))
                    .route("/create", web::post().to(textbook_handler::create_textbook))
                    .route("/delete", web::post().to(textbook_handler::delete_textbook))
                    .route(
                        "/units",
                        web::post().to(textbook_handler::get_unit_by_textbook),
                    ),
            )
            .service(
                web::scope("/unit")
                    .route("/create", web::post().to(unit_handler::create_unit))
                    .route("/list", web::post().to(unit_handler::get_units))
                    .route("/delete", web::post().to(unit_handler::delete_unit))
                    .route(
                        "/words",
                        web::post().to(word_unit_mapping_handler::get_unit_words),
                    )
                    .route(
                        "/createWord",
                        web::post().to(word_unit_mapping_handler::create_unit_word),
                    )
                    .route(
                        "/deleteWord",
                        web::post().to(word_unit_mapping_handler::delete_unit_word),
                    ),
            ),
    );
}
