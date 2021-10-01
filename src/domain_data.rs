use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AvailableFVABank {
    pub name: String,
    pub code: String,
    pub is_activated: bool,
}
