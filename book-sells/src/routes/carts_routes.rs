use actix_web::web;
use crate::handler::cart_handler::{pro_file, get_cart_item_by_id};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(pro_file);
    cfg.service(get_cart_item_by_id);
}