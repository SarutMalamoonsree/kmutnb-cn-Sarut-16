use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct List {
    pub id: i32,
    pub name: String,
}