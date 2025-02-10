use rand::thread_rng;
use std::collections::HashSet;
use rand::prelude::IndexedRandom; 

fn generate_fruit() -> &'static str {
    let fruits  = [
        "apple", 
        "banana", 
        "cherry", 
        "date", 
        "elderberry", 
        "fig", 
        "grape", 
        "honeydew", 
        "kiwi", 
        "lemon", 
        "mango", 
        "nectarine", 
        "orange", 
        "papaya", 
        "quince", 
        "raspberry", 
        "strawberry", 
        "tangerine", 
        "ugli fruit", 
        "watermelon"
    ];

    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 random fruits");
    for _ in 0..100 {
        fruit_set.insert( generate_fruit() );
    }
    
    println!("Number of unique fruits generated: {}", fruit_set.len());
}
