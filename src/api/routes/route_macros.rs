#[macro_export]
macro_rules! define_routes {
    ($handler:ty, $($method:ident $path:expr => $handler_fn:path),* $(,)?) => {
        impl crate::api::handler::Handler for $handler {
            fn configure(cfg: &mut actix_web::web::ServiceConfig) {
                use actix_web::web;
                $(
                    cfg.service(
                        web::resource($path)
                            .$method($handler_fn)
                    );
                )*
            }
        }
    };
}
