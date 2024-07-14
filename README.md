# [crate link](https://crates.io/crates/EasyAlgolia)

# EasyAlgolia is a Rust crate designed for utilizing the Algolia admin client. It simplifies the process of updating and inserting documents into Algolia's search index.

### this can also be used as database to store simple Json databased in algolia 

![Alt text](https://upload.wikimedia.org/wikipedia/commons/thumb/d/da/Algolia_logo.svg/1200px-Algolia_logo.svg.png "a title")


### features
* custom Object and trait 
* supoort async and sync operations


###  Usage 
* using raw json
```rust
    #[tokio::main]
    async fn main() -> Result<(), EasyAlgoliaError> {
        dotenv().ok();
        let client = ClientBuilder::build_from_env()?;
        // for raw values, Object ids are provided from algolia or can be explicitly put into json document
        let data = serde_json::json!({
            "name":" Hello world ! ",
            "about":" i love rust " ,
            "objectID" : "123456"
        });

        let my_index: Index = "Test".into();
        client.put_document_async(&my_index, data).await?;
        Ok(())
}
  ```
* using user defined struct 
```rust
     #[derive(Default)]
     struct MyObject{
        name:String,
        class:i32,
        course:String
    }
    impl EasyAlgolia::algoliaobject::AlgoliaObject for MyObject{}

    #[tokio::main]
    async fn main() -> Result<(), EasyAlgoliaError> {
        dotenv().ok();
        let client = ClientBuilder::build_from_env()?;
        let doc = MyObject::default();
        let user_index: Index = "Users".into();
        client.put_document_async(&my_index, data).await?;
        Ok(())
}
  ```

# todo
### Admin API key Curd operation
### Delete by Query
### Get Objects from Index
