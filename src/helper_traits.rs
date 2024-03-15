///! Algolia Index object
pub trait ObjectId: serde::Serialize {
    fn get_object_id(&self) -> Option<String> {
        None
    }
}
