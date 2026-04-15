use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

//this is a macro, it is useful as it gives the compiler the ability to deep copy() the struct, print it in debug statements and convert it from JSON to Point and viceversa
#[derive(Debug, Clone, Serialize, Deserialize)]
//this is how structs are defined (like types and interfaces of typescript)
pub struct Point {
    pub id: Uuid,
    // the payload is an unknown JSON format so it has to be Option<Value> for metadata assignation
    pub payload: Option<Value>,
    pub vector: Vec<f32>,
}

#[derive(Debug, Deserialize)]
pub struct InsertRequest {
    pub points: Vec<Point>,
}

