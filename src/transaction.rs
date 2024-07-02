use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NFTTransaction {
    pub first_name: String,
    pub last_name: String,
    pub class_number: String,
    pub image_hash: String,
}

impl NFTTransaction {
    pub fn new(first_name: String, last_name: String, class_number: String, image_hash: String) -> Self {
        NFTTransaction { first_name, last_name, class_number, image_hash }
    }
}
