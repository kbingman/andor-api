use anyhow::Result;

/// A Trait for connect arbitrary databases with CRUD actions
/// This connect a Model struct to a given DB and describes a
/// generic interface for connecting to the DB
pub trait DbAdapter<Model> {
    fn insert(&self, model: &Model) -> Result<Option<Model>>;
    fn find_all(&self) -> Result<Vec<Model>>;
    fn find_one(&self, id: i32) -> Result<Option<Model>>;
    fn update(&self, id: i32, model: &Model) -> Result<Option<Model>>;
    fn delete(&self, id: i32) -> Result<u64>;
}
