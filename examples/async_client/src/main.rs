use EasyAlgolia::{
    client_builder::ClientBuilder,
    error::EasyAlgoliaError,
    Index,
};
use dotenv::dotenv;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), EasyAlgoliaError> {
    dotenv().ok();
    let client = ClientBuilder::build_from_env()?;
    
    // for raw values, Object ids are provided from algolia or can be explicitly put into json document body
    let data = serde_json::json!({
        "name":" Hello world ! ",
        "about":" i love rust " ,
        "objectID" : "123456"
    });

    let my_index: Index = "Test".into();
    let a = client.put_document_async(&my_index, data).await;


    Ok(())
}