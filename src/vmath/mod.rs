pub fn euclidian_distance(vector1: Vec<f32>, vector2: Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for i in 0..vector1.len() {
        sum += (vector1[i] - vector2[i]).powi(2);
    }
    return sum.sqrt()
}

pub fn cosine_similarity(vector1: Vec<f32>, vector2: Vec<f32>) -> f32 {
    let mut dot_product = 0.0;
    let mut magnitude1 = 0.0;
    let mut magnitude2 = 0.0;
    for i in 0..vector1.len() {
        dot_product += vector1[i] * vector2[i];
        magnitude1 += vector1[i].powi(2);
        magnitude2 += vector2[i].powi(2);
    }
    return dot_product / (magnitude1.sqrt() * magnitude2.sqrt())
}