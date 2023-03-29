use actix_web::{get, Responder, HttpResponse};
use serde_json::json;

use crate::models::product::Books;
use crate::models::profile::Profile;
use crate::models::list::List;


#[get("/profile")]
async fn pro_file() -> impl Responder {
    let id: i32 = 20;
    let profile = vec![
        Profile {
            id: id,
            username: "Pond".to_string()
        }
    ];
    let response_body = json!(profile);

    HttpResponse::Ok().json(response_body)
}

#[get("/cart")]
async fn get_cart_item_by_id() -> impl Responder {
    let list = vec![
            List {
            id: 1,
            name: "Little Red Riding Hood".to_string(),
         },
            List { 
            id: 2,
            name: "Peterpan".to_string(),
         } 
    ];
    let list2 = vec![
            List {
            id: 5,
            name: "คณิตไม่ต้องคิด".to_string(),
         },
            List { 
            id: 6,
            name: "ไทยวิบัติ".to_string(),
            
         } 
    ];
    let product_item = vec![
        Books {
            catagory: "kid".to_string(),
            published_at: "2023-03-19T08:40:51.620Z".to_string(),
            list : list
        },
        Books {
            catagory: "education".to_string(),
            published_at: "2023-03-19T08:50:51.620Z".to_string(),
            list : list2
        } 
    ];
    let response_body = json!(product_item);
    
    HttpResponse::Ok().json(response_body)
    
}