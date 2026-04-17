use crate::storage::Collection;
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::models::Point;
use tokio::sync::RwLockWriteGuard;
use tokio::sync::RwLockReadGuard;

#[derive(Clone)]
pub struct Database {
    pub collections: Arc<RwLock<HashMap<String, Collection>>>,

}

impl Database {
    pub fn new() -> Self {
        Database {
            collections: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    //this function aquires a T type, it can be anything that can be converted to a String, so when returning an Error we can parse it to a string to match the function signature and prevent E0207
    pub async fn create_collection(&self, name: String, dimension: usize) -> Result<&str, &str> {
        let mut collections: RwLockWriteGuard<HashMap<String, Collection>> = self.collections.write().await;
        if collections.contains_key(&name.to_string()) {
            return Err("Collection already exists");
        }
        collections.insert(name.to_string(), Collection::new(name.clone(), dimension));
        return Ok("collection added successfully");
    }
    pub async fn search_collection(&self, collection_name: String, query_vector: Vec<f32>, k: usize) -> Result<Vec<Point>, &str> {
        let collections: RwLockReadGuard<HashMap<String, Collection>> = self.collections.read().await;
        let collection: Result<&Collection, &str> = collections.get(&collection_name).ok_or("Collection not found");

        let results: Result<Vec<Point>, &str> = collection?.search(query_vector, k);
        return Ok(results.into_iter().flatten().collect());
    }
}