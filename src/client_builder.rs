//! Client builder for Algolia client
//! Should be used when need to build client with custom setting
//! can be used to build client from environment variables

use crate::error::EasyAlgoliaError;
use crate::{error::ErrorKind, Client};
use std::mem;

use secrecy::{ExposeSecret, Secret};
pub struct ClientBuilder {
    application_id: Option<Secret<String>>,
    api_key: Option<Secret<String>>,
}

impl ClientBuilder {
    /// create a new client builder with credentials set to None
    /// calling build on unset variables client builder will result in [EasyAlgoliaError](crate::error::EasyAlgoliaError)
    // # Examples
    /// ```
    /// use EasyAlgolia::client_builder::ClientBuilder ;
    /// let algolia_client_builder:ClientBuilder = ClientBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            application_id: None,
            api_key: None,
        }
    }

    /// add application id to the client builder
    /// # Arguments
    /// * `application_id` - original string reference.
    /// # Examples
    /// ```
    /// use EasyAlgolia::client_builder::ClientBuilder ;
    /// let mut algolia_client_builder:ClientBuilder = ClientBuilder::new().set_application_id("123") ;
    /// ```
    pub fn set_application_id(mut self, app_id: &str) -> Self {
        self.application_id = Some(Secret::new(String::from(app_id)));
        self
    }
    /// add api to the client builder
    /// # Arguments
    ///
    /// * `api_key` - original string reference.
    ///
    /// # Examples
    /// ```
    /// use EasyAlgolia::client_builder::ClientBuilder ;
    /// let algolia_client_builder:ClientBuilder = ClientBuilder::new().set_api_key("123") ;
    /// ```
    pub fn set_api_key(mut self, app_id: &str) -> Self {
        self.api_key = Some(Secret::new(String::from(app_id)));
        self
    }

    /// build the client from store credentials
    /// if api_key and app_id are set, function will consume them and set None
    /// returns error if either are not set
    /// # Examples
    /// ```
    /// use EasyAlgolia::client_builder::ClientBuilder ;
    /// let mut algolia_client = ClientBuilder::new()
    ///                         .set_application_id("123")
    ///                         .set_api_key("123").build().unwrap();
    /// ```
    /// # Error
    /// returns [`Err(EasyAlgoliaError)`](crate::error::EasyAlgoliaError) if either of the application_id or api_key is not set
    /// ```panics
    /// use EasyAlgolia::client_builder::ClientBuilder ;
    /// // result in panic
    /// let mut algolia_client = ClientBuilder::new().build().unwrap();
    /// ```

    pub fn build<'a>(&mut self) -> Result<Client, EasyAlgoliaError<'a>> {
        if self.api_key.is_some() && self.application_id.is_some() {
            let api_key = mem::take(&mut self.api_key);
            let application_id = mem::take(&mut self.application_id);
            return Ok(Client::new(
                application_id.unwrap().expose_secret(),
                api_key.unwrap().expose_secret(),
            ));
        } else {
            Err(EasyAlgoliaError::new(
                ErrorKind::ClientBuilderError,
                Some(" unable to fetch client id or api key "),
            ))
        }
    }

    /// build the client from environment variables
    /// if api_key and app_id are set, `Err(EasyAlgoliaError)` will be returned
    /// returns error if either are not set
    /// # Environment variables
    /// `APPLICATION_ID` and `APP_ID`
    /// # Examples
    /// ```ignore
    /// let mut algolia_client = ClientBuilder::build_from_env().unwrap();
    /// ```

    pub fn build_from_env<'a>()-> Result<Client, EasyAlgoliaError<'a>>{

       todo!()
    }
}
