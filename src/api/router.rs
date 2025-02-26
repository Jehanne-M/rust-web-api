use actix_web::web;

use crate::api::routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(routes::user::config));
}
