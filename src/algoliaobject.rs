//! Algolia Object Id trait
/// All object passed to `update_document` or `insert_document` method of `Client` object must impl
/// AlgoliaObject trait
pub trait AlgoliaObject: serde::Serialize {
    /// Client calls this function to get the object Id of the document
    /// By default algolia itself provides Object id for each document getting inserted
    /// but this behaviour can be overridden by implementing AlgoliaObject 'get_object_id' method
    /// # Examples
    /// ```ignore
    ///    use EasyAlgolia::traits::AlgoliaObject;
    ///    use serde::Serialize ;
    ///    #[derive(Serialize)]
    ///    struct MyObject{
    ///      name:String,
    ///      location:String,
    ///      age:i32
    ///    }
    ///    
    ///  fn main(){
    ///    
    ///    // if you want default object from Algolia for your object
    ///    impl AlgoliaObject for MyObject ;
    ///     
    ///    // if you want a custom object id for your object
    ///    
    ///    impl AlgoliaObject for MyObject {
    ///       fn get_object_id(&self) -> Option<String> {
    ///          get_my_object_id(&self)
    ///          
    ///     }
    /// }  
    /// ```
    fn get_object_id(&self) -> String {
        "".into()
    }
}

impl AlgoliaObject for serde_json::value::Value {}
