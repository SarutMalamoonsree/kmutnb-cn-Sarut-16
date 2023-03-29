use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Books {
    pub catagory: String,
    pub published_at: String,
    pub list: vec![List]
}

