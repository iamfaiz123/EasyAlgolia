#[cfg(test)]
mod test {
    use easyalgolia::client_builder::ClientBuilder;
    // Import the ClientBuilder type from the module where it's defined

    #[test]
    fn test_client_builder() {
        let app_id = "123";
        let _ = ClientBuilder::new()
            .set_api_key(app_id)
            .set_application_id(app_id);
        // Add assertions or other test logic here
        assert!(true)
    }
}
