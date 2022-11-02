use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Reply{
    pub message: String,
}
