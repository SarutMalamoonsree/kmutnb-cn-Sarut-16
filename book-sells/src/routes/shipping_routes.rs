use actix_web::web;
use crate::handler::shipping_handlers::get_cart;


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_cart);
}