//! Easy Algolia is unofficial Rust client for algolia admin to update and insert data in Algolia Search Engine

pub mod client_builder;
pub mod error;
use secrecy::Secret;
pub mod helper_traits;
use crate::helper_traits::ObjectId;
pub struct Client{
    api_key: Secret<String>,
    application_id: Secret<String>,
}

impl Client {
    

    pub fn new(api_key: &str, application_id: &str) -> Self {
             Self { api_key: Secret::new(String::from(api_key)), application_id: Secret::new(String::from(api_key)) }
    }
    
    pub async fn update_document<T>(&self, index: &String, document: T)
    where
        T: serde::Serialize + ObjectId,
    {
        // let mut client = self.client.put(format!(
        //     "https://{}.algolia.net/1/indexes/{}/{}",
        //     &self.application_id,
        //     index,
        //     document.get_object_id()
        // ));
        // client = client.header("X-Algolia-API-Key", &self.api_key);
        // client = client.header("X-Algolia-Application-Id", &self.application_id);
        // client = client.json(&document);
        // match client.send().await {
        //     Ok(_) => {
        //     }
        //     Err(err) => {
        //         todo!()
        //     }
        // }
    }
    pub async fn delete_document<T>(&self, index: &String, document: T)
    where
        T: serde::Serialize + ObjectId,
    {
        // let mut client = self.client.delete(format!(
        //     "https://{}.algolia.net/1/indexes/{}/{}",
        //     &self.application_id,
        //     index,
        //     document.get_object_id()
        // ));
        // client = client.header("X-Algolia-API-Key", &self.api_key);
        // client = client.header("X-Algolia-Application-Id", &self.application_id);
        // // client = client.json(&document);
        // match client.send().await {
        //     Ok(_) => {}
        //     Err(err) => {
        //         todo!()
        //     }
        // }
    }
}
