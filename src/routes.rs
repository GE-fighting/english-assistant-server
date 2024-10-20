use crate::handlers::user_handler;
use crate::handlers::word_handler;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/greet").route(web::get().to(user_handler::greet)))
            .service(
                web::scope("/words")
                    .route("insert", web::post().to(word_handler::create_word))
                    .route("get", web::post().to(word_handler::get_word))
                    .route(
                        "inner_update",
                        web::post().to(word_handler::update_batch_words),
                    ),
            ),
    );
}
