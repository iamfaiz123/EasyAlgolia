use EasyAlgolia::{error::EasyAlgoliaError ,client_builder::ClientBuilder};
fn main()->Result<(),EasyAlgoliaError<'static >> {
    let application_id = "some random application id";
    let app_id = " some random app id";
    let _client = ClientBuilder::new()
        .set_api_key(app_id)
        .set_application_id(application_id)
        .build()?;

    Ok(())
}
