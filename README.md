# [crate link](https://crates.io/crates/EasyAlgolia)

# EasyAlgolia is a Rust crate designed for utilizing the Algolia admin client. It simplifies the process of updating and inserting documents into Algolia's search index.

![Alt text](https://upload.wikimedia.org/wikipedia/commons/thumb/d/da/Algolia_logo.svg/1200px-Algolia_logo.svg.png "a title")

# This crate is still in development 

###  Usage 
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

# todo
### Admin API key Curd operation
### Delete by Query
### Get Objects from Index