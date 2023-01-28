use anyhow::Result;

pub trait DbAdapter<Model, Response> {
    fn insert(&self, model: &Model) -> Result<Response>;
    fn find_all(&self) -> Result<Response>;
    fn find_one(&self, id: i32) -> Result<Response>;
    fn update(&self, id: i32, model: &Model) -> Result<Response>;
    fn delete(&self, id: i32) -> Result<u64>;
}
