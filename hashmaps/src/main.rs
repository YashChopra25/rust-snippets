use std::collections::HashMap;
fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    let key = String::from("age");
    scores.insert(key.clone(), 21);
    let val = scores.get(&key).copied().unwrap_or(0);
    println!("Hello, world! {:?} or {}", scores, val)
}
