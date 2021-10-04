use crate::domain_data::{AvailableFVABank, CreateInvoiceFaVAParameter, VirtualAccount};
use reqwest::Client;

const XENDIT_API: &str = "https://api.xendit.co";

#[derive(Clone)]
pub struct Xendit {
    pub secret_api_key: String,
    pub http: Client,
}

#[derive(Debug)]
pub enum XenditError {
    RequestError(reqwest::Error),
    ParseError(serde_json::Error),
    LibraryError(String),
}

impl Xendit {
    pub fn new(secret_api_key: String) -> Xendit {
        Xendit {
            secret_api_key,
            http: Client::default(),
        }
    }

    fn get_key(&self) -> &String {
        &self.secret_api_key
    }

    fn http(&self) -> &Client {
        &self.http
    }

    /// Check available FVA Banks
    pub async fn list_fva_banks(&self) -> Result<Vec<AvailableFVABank>, XenditError> {
        let url: String = XENDIT_API.to_owned() + "/available_virtual_account_banks";
        match self
            .http()
            .get(url)
            .basic_auth(self.get_key(), Some(""))
            .send()
            .await
        {
            Ok(res) => {
                let res = res.json::<Vec<AvailableFVABank>>().await;
                match res {
                    Ok(res) => Ok(res),
                    Err(e) => Err(XenditError::RequestError(e)),
                }
            }
            Err(e) => Err(XenditError::RequestError(e)),
        }
    }

    /// Check the virtual account detail and status
    pub async fn virtual_account_detail(&self, va_id: &str) -> Result<VirtualAccount, XenditError> {
        let url: String = XENDIT_API.to_owned() + "/callback_virtual_accounts/" + va_id;
        match self
            .http()
            .get(url)
            .basic_auth(self.get_key(), Some(""))
            .send()
            .await
        {
            Ok(res) => {
                let res = res.json::<VirtualAccount>().await;
                match res {
                    Ok(res) => Ok(res),
                    Err(e) => Err(XenditError::RequestError(e)),
                }
            }
            Err(e) => Err(XenditError::RequestError(e)),
        }
    }
}
