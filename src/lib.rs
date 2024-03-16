
//! Easy Algolia is unofficial Rust client for algolia admin to update and insert data in Algolia
//! Search Engine

#[allow(non_snake_case)]
pub mod client_builder;
pub mod error;
use error::EasyAlgoliaError;
use secrecy::{
    ExposeSecret,
    Secret,
};
pub mod algoliaobject;
use crate::algoliaobject::AlgoliaObject;
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

/// Client to interact with algolia
pub struct Client {
    api_key: Secret<String>,
    application_id: Secret<String>,
    client: Rq,
}

impl Client {
    pub(crate) fn new(api_key: &str, application_id: &str) -> Self {
        Self {
            api_key: Secret::new(String::from(api_key)),
            application_id: Secret::new(String::from(application_id)),
            client: Rq::new(),
        }
    }
    /// update or insert a data into given algolia index
    /// if the contained document objectId is present in algolia index, then this function will update document with new values
    /// if the document does default impls the implmentation AlgoliaObject , it will insert new doc of the given value in index
    /// # Arguments
    /// [EasyAlgoliaError](crate::Index)
    /// 
    // # Examples
    /// ```ignore
    /// use EasyAlgolia::trait::AlgoliaObject;
    /// #[derive(serde::Serialize)]
    /// struct MyDoc{
    ///  pub obj_id:String,
    ///  pub name:String,
    ///  pub class:i32,
    ///  pub course:String
    ///  }
    /// 
    /// impl AlgoliaObject for MyDoc{
    ///   fn get_object_id(&self) -> String {
    ///   &self.obj_id.into()
    /// }
    ///  let doc = MyDoc{
    ///   obj_id: String::from("some obj id") ,
    ///   ..Default::default()
    ///   } ;
    ///  client.put_document_async("someIndex".into(),doc)
    /// ```
    
    pub async fn put_document_async<'a,T: AlgoliaObject>(
        &self,
        index: &Index,
        document: &T,
    ) -> Result<(), EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {   
        let mut is_object_is_present = false;
        let path = match document.get_object_id().as_str() {
            // if object id is not present in algolia doc then put random object id
            // random id is generted by algolia
            "" => {
                format!(
                    "https://{}.algolia.net/1/indexes/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                )
            }
            _ => {
                is_object_is_present = true ;
                format!(
                    "https://{}.algolia.net/1/indexes/{}/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                    document.get_object_id()
                )
            }
        };
        let mut client = match is_object_is_present {
            true => self.client.put(path) ,
            false => self.client.post(path) 
        } ;
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        client = client.json(&document);

        match client.send().await {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap() || k.status() < reqwest::StatusCode::from_u16(200).unwrap() {
                    return Err(
                        EasyAlgoliaError::new(
                            error::ErrorKind::RequestError,
                            Some(k.text().await.unwrap())
                        )
                    ) ;
                }

                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }
    // pub async fn delete_document<T>(&self, index: &String, document: T)
    // where
    //     T: serde::Serialize + AlgoliaObject,
    // {
    //     // let mut client = self.client.delete(format!(
    //     //     "https://{}.algolia.net/1/indexes/{}/{}",
    //     //     &self.application_id,
    //     //     index,
    //     //     document.get_object_id()
    //     // ));
    //     // client = client.header("X-Algolia-API-Key", &self.api_key);
    //     // client = client.header("X-Algolia-Application-Id", &self.application_id);
    //     // // client = client.json(&document);
    //     // match client.send().await {
    //     //     Ok(_) => {}
    //     //     Err(err) => {
    //     //         todo!()
    //     //     }
    //     // }
    // }
}
