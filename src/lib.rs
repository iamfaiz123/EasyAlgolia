//! Easy Algolia is unofficial Rust client for algolia admin to update and insert data in Algolia
//! Search Engine

pub mod client_builder;
pub mod error;
use error::EasyAlgoliaError;
use secrecy::{
    ExposeSecret,
    Secret,
};
pub mod traits;
use crate::traits::AlgoliaObject;
use reqwest::Client as Rq;

/// index object to store the index of the Algoia
pub struct Index {
    index: String,
}
impl Index {
    fn index(&self) -> &str {
        &self.index
    }
}

impl From<String> for Index {
    fn from(s: String) -> Self {
        Self { index: s }
    }
}

impl From<&str> for Index {
    fn from(s: &str) -> Self {
        Self {
            index: String::from(s),
        }
    }
}

pub struct Client {
    api_key: Secret<String>,
    application_id: Secret<String>,
    client: Rq,
}

impl Client {
    pub fn new(api_key: &str, application_id: &str) -> Self {
        Self {
            api_key: Secret::new(String::from(api_key)),
            application_id: Secret::new(String::from(application_id)),
            client: Rq::new(),
        }
    }

    pub async fn update_document_async<T: AlgoliaObject>(
        &self,
        index: &Index,
        document: T,
    ) -> Result<(), EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {
        let path = match index.index() {
            // if object id is not present in algolia doc then put random object id
            // random id is generted by algolia
            "" => {
                format!(
                    "https://{}.algolia.net/1/indexes/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                    // document.get_object_id()
                )
            }
            _ => {
                format!(
                    "https://{}.algolia.net/1/indexes/{}/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                    document.get_object_id()
                )
            }
        };

        let mut client = self.client.put(path);
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        client = client.json(&document);

        match client.send().await {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => Err(err.into()),
        }
    }
    pub async fn delete_document<T>(&self, index: &String, document: T)
    where
        T: serde::Serialize + AlgoliaObject,
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
