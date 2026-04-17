//if models arent imported compiler ignores them
mod models;
mod vmath;
mod storage;
mod engine;

fn main() {
    //this searches for mod.rs files in subfolders wich is a cool way to import them, nice rust!
    use vmath::euclidian_distance;
    use vmath::cosine_similarity;

    #[tokio::main]
    async fn main() {
        println!("Victooor!");
    }
}
