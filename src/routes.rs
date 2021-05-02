//! Place all Actix routes here, multiple route configs can be used and
//! combined.
use crate::handlers::form::{
    form_get, submit_buy, submit_reg_bu, submit_reg_ps, submit_repair, submit_trail,
};
use actix_files::Files;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // form routine
        .service(
            web::scope("/form")
                .service(form_get)
                .service(submit_trail)
                .service(submit_buy)
                .service(submit_reg_bu)
                .service(submit_reg_ps)
                .service(submit_repair),
        )
        // Serve secure static files from the static-private folder
        .service(Files::new("/asset", "static/asset/").show_files_listing())
        .service(Files::new("/", "static/html/").index_file("index.html"));
}
