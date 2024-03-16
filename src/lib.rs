//! easy Algolia is unofficial Rust client for algolia admin to update and insert data in Algolia
//! Search Engine
#[allow(non_snake_case)]
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

/// setting for Algolia Index
/// This seeting can be loaded and can bet set from backend using
/// [client.update_index_setting](crate::Client::update_index_setting) method ```ignore
///    let my_index:Index = "MyIndex".into() ;
///    let mut setting = client.get_index_setting(my_index).await;
///    // set default setting for index
///    let new_setting = AlgoliaIndexSetting::Default();
///    // update setting
///    client.update_index_setting(my_index,new_setting)
/// ```
///    
#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AlgoliaIndexSetting {
    pub min_word_size_for_1_typo: u32,
    pub min_word_size_for_2_typos: u32,
    pub hits_per_page: u32,
    pub max_values_per_facet: u32,
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_attributes_to_index: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_retrieve: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unretrievable_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_words: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_for_faceting: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_snippet: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_to_highlight: Option<Vec<String>>,
    pub pagination_limited_to: u32,
    pub attribute_for_distinct: Option<String>,
    pub exact_on_single_word_query: String,
    pub ranking: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ranking: Option<Vec<String>>,
    pub separators_to_index: String,
    pub remove_words_if_no_results: String,
    pub query_type: String,
    pub highlight_pre_tag: String,
    pub highlight_post_tag: String,
    pub alternatives_as_exact: Vec<String>,
}

/// object Id struct for docs delete
/// This struct can be used to delete pre-existing docs in algoia
/// original self described document can be passed as well ( since it impls AlgoliaObj trait ) , but
/// since it consume the document its developers choice how to procced # Usage
/// ```ignore
///  // suppose you already have a document in Index named Games and with id 'god-of-war'
///  // to delete that document from that index you can use
///  let obj_id:ObjectId = "god-of-war".into();
///  let index:Index = "Games".into() ;
///  let client = Clientbuilder::build_from_env().unwarp();
///  client.delete_document_async(index,obj_id).await.unwarp();
///  ```

#[derive(serde::Serialize)]
pub struct ObjectId {
    obj_id: String,
}
impl From<String> for ObjectId {
    fn from(s: String) -> Self {
        Self { obj_id: s }
    }
}

impl From<&str> for ObjectId {
    fn from(s: &str) -> Self {
        Self {
            obj_id: String::from(s),
        }
    }
}

impl AlgoliaObject for ObjectId {
    fn get_object_id(&self) -> String {
        String::from(&self.obj_id)
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
    /// if the contained document objectId is present in algolia index, then this function will
    /// update document with new values if the document does default impls the implmentation
    /// AlgoliaObject , it will insert new doc of the given value in index # Arguments
    /// [EasyAlgoliaError](crate::Index)
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

    pub async fn put_document_async<T: AlgoliaObject>(
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
                is_object_is_present = true;
                format!(
                    "https://{}.algolia.net/1/indexes/{}/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                    document.get_object_id()
                )
            }
        };
        let mut client = match is_object_is_present {
            true => self.client.put(path),
            false => self.client.post(path),
        };
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        client = client.json(&document);

        match client.send().await {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(k.text().await.unwrap()),
                    ));
                }

                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// same as [put_document_async](crate::Client::put_document_async) but blocking in nature
    /// /// under the hood it still uses asyn reqwest method only , but the runtime is block by
    /// `futures::executor::block_on`
    pub fn put_document<T: AlgoliaObject>(
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
                is_object_is_present = true;
                format!(
                    "https://{}.algolia.net/1/indexes/{}/{}",
                    &self.application_id.expose_secret(),
                    index.index(),
                    document.get_object_id()
                )
            }
        };
        let mut client = match is_object_is_present {
            true => self.client.put(path),
            false => self.client.post(path),
        };
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        client = client.json(&document);

        match futures::executor::block_on(client.send()) {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(futures::executor::block_on(k.text()).unwrap()),
                    ));
                }

                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// update or insert a data into given algolia index
    /// if the contained document objectId is present in algolia index, then this function will
    /// update document with new values if the document does default impls the implmentation
    /// AlgoliaObject , it will insert new doc of the given value in index # Arguments
    /// [EasyAlgoliaError](crate::Index)
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
    pub async fn delete_document_async<T>(
        &self,
        index: &Index,
        document: T,
    ) -> Result<(), EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {
        let path = match document.get_object_id().as_str() {
            "" => {
                return Err(EasyAlgoliaError::new(
                    error::ErrorKind::RequestError,
                    Some("object id must be present for document delete method".into()),
                ));
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

        let mut client = self.client.delete(path);
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        match client.send().await {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(k.text().await.unwrap()),
                    ));
                }

                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// same as [delete_document_async](crate::Client::delete_document_async) but its synchronous in
    /// nature under the hood it still uses asyn reqwest method only , but the runtime is block
    /// by `futures::executor::block_on`
    pub fn delete_document<T>(&self, index: &Index, document: T) -> Result<(), EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {
        let path = match document.get_object_id().as_str() {
            "" => {
                return Err(EasyAlgoliaError::new(
                    error::ErrorKind::RequestError,
                    Some("object id must be present for document delete method".into()),
                ));
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

        let mut client = self.client.delete(path);
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        match futures::executor::block_on(client.send()) {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(futures::executor::block_on(k.text()).unwrap()),
                    ));
                }

                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// get settings for a given index
    /// return setting from this function can be modfied and set, key sets to None will be replaced
    /// to default ```ignore
    ///    let index:Index = "SomeIndex".into();
    ///    let setting = client.get_index_setting().await?;
    /// ```
    pub async fn get_index_setting<T: AlgoliaObject>(
        &self,
        index: &Index,
    ) -> Result<AlgoliaIndexSetting, EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {
        let path = format!(
            "https://{}.algolia.net/1/indexes/{}/settings",
            &self.application_id.expose_secret(),
            index.index()
        );
        let mut client = self.client.get(path);
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        match client.send().await {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(k.text().await.unwrap()),
                    ));
                } else {
                    let setting: AlgoliaIndexSetting = k
                        .json::<AlgoliaIndexSetting>()
                        .await
                        .map_err(|_| EasyAlgoliaError::new(error::ErrorKind::RequestError, None))?;

                    Ok(setting)
                }
            }
            Err(err) => Err(err.into()),
        }
    }

    /// upload settings for a given index
    /// key sets to None will be replaced to default
    /// ```ignore
    ///    let index:Index = "SomeIndex".into();
    ///    let index_setting = AlgoliaIndexSetting::default();
    ///    let setting = client.update_index_setting().await?;
    /// ```
    pub async fn update_index_setting<T: AlgoliaObject>(
        &self,
        index: &Index,
        setting: AlgoliaIndexSetting,
    ) -> Result<(), EasyAlgoliaError>
    where
        T: serde::Serialize + AlgoliaObject,
    {
        let path = format!(
            "https://{}.algolia.net/1/indexes/{}/settings",
            &self.application_id.expose_secret(),
            index.index()
        );
        let mut client = self.client.put(path);
        client = client.header("X-Algolia-API-Key", self.api_key.expose_secret());
        client = client.header(
            "X-Algolia-Application-Id",
            self.application_id.expose_secret(),
        );
        client = client.json(&setting);
        match client.send().await {
            Ok(k) => {
                if k.status() > reqwest::StatusCode::from_u16(200).unwrap()
                    || k.status() < reqwest::StatusCode::from_u16(200).unwrap()
                {
                    return Err(EasyAlgoliaError::new(
                        error::ErrorKind::RequestError,
                        Some(k.text().await.unwrap()),
                    ));
                } else {
                    Ok(())
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}
