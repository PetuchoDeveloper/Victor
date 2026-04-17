use crate::models::Point;
use std::collections::HashMap;
use crate::vmath::{euclidian_distance, cosine_similarity};
use uuid::Uuid;
use tokio::fs;

//in rust behavior and data are separated, the struct defines the shape of the object: what does it contain?
#[derive(Clone, Debug)]
pub struct Collection {
    pub name: String,
    pub dimension: usize,
    pub metric: String,
    pub hmap: HashMap<Uuid, Point>,
}
//while an implementation defines the behavior of the object, it can define default values for the structure and what methods are available to read or mutate the data inside the structure: what it does?
impl Collection {
    //this, besides being the constructor, is also called an associated function, these dont take self as an argument but are called directly on the struct type, these are usually used to create new instances of the struct
    pub fn new(name: String, dimension: usize) -> Self {
        Collection {
            name: name,
            dimension: dimension,
            metric: String::new(),
            hmap: HashMap::new(),
        }
    }
    //these are methods, they can define if they mutate the structure somehow or if they are read-only, but regardless they do work, like standard methods from other languages
    pub fn upsert(&mut self, point: Point) -> Result<&str, &str> {
        if point.vector.len() != self.dimension {
            return Err("Vector dimension does not match collection dimension");
        }
        self.hmap.insert(point.id, point);
        return Ok("vector upserted");
    }
    pub fn search(&self, query_vector: Vec<f32>, k: usize) -> Result<Vec<Point>, &str> {
        if query_vector.len() != self.dimension {
            return Err("Vector dimension does not match collection dimension");
        }
        let mut distances = Vec::new();
        
        for point in self.hmap.values() {
            match self.metric.as_str() {
                "euclidean" => {
                    distances.push((point.id, euclidian_distance(point.vector.clone(), query_vector.clone())));
                }
                "cosine" => {
                    distances.push((point.id, cosine_similarity(point.vector.clone(), query_vector.clone())));
                }
                _ => {
                    return Err("Invalid metric");
                }
            }
        }
        match self.metric.as_str() {
            "euclidean" => {
                distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            }
            "cosine" => {
                distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            }
            _ => {
                return Err("Invalid metric");
            }
        }

        distances.truncate(k);
        println!("{:?}", distances);
        let results: Vec<Point> = distances.into_iter().map(|(id, _)| self.hmap.get(&id).unwrap().clone()).collect::<Vec<Point>>();
        return Ok(results);
    }

    pub async fn save_to_disk(&self, filepath: &str) -> Result<&str, String> {
        let serialized_data = serde_json::to_string(&self.hmap).map_err(|e| e.to_string())?;

        fs::write(filepath, serialized_data).await.map_err(|e| e.to_string())?;

        return Ok("vectors saved to disk");
    }
        
}