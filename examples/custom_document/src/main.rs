
use EasyAlgolia::{
    algoliaobject::AlgoliaObject,
    client_builder::ClientBuilder,
    error::EasyAlgoliaError,
    Index,
};
use dotenvy::dotenv;

#[derive(serde::Serialize, Default)]
struct Game {
    pub name: String,
    genre: String,
    platform: String,
    release_year: u32,
    is_multiplayer: bool,
}
impl AlgoliaObject for Game {
    fn get_object_id(&self) -> String {
        String::from(&self.name)
    }
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), EasyAlgoliaError> {
    dotenv().ok();
    let client = ClientBuilder::build_from_env()?;

    let my_index: Index = "Test".into();
    let doc = Game { name:"LastO fUs".into(), ..Default::default() } ;
    // if document is not present in the index, this will insert a new document
    let _ = client.put_document_async(&my_index, &doc).await?;

    // calling the same function again with same document, ie doc.get_object_id = "LastOfUs" will update the docmuent
    let _ = client.put_document_async(&my_index, &doc).await?;


    Ok(())
}
