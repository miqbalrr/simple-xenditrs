use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct AvailableFVABank {
    pub name: String,
    pub code: String,
    pub is_activated: bool,
}
#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct VirtualAccount {
    pub id: String,
    pub owner_id: String,
    pub external_id: String,
    pub account_number: String,
    pub bank_code: String,
    pub merchant_code: String,
    pub name: String,
    pub is_closed: bool,
    pub expiration_date: String,
    pub is_single_use: bool,
    pub status: String,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub struct CreateInvoiceFaVAParameter {
    pub external_id: String,
    pub amount: i32,
    pub payer_email: String,
    pub description: String,
    pub callback_virtual_account_id: String,
}
