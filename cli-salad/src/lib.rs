use rand::seq::SliceRandom;
use rand::rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let mut fruits: Vec<String> = vec![
        "apple".to_string(), 
        "banana".to_string(), 
        "cherry".to_string(), 
        "date".to_string(), 
        "elderberry".to_string(), 
        "fig".to_string(), 
        "grape".to_string(), 
        "honeydew".to_string(), 
        "kiwi".to_string(), 
        "lemon".to_string(), 
        "mango".to_string(), 
        "nectarine".to_string(), 
        "orange".to_string(), 
        "papaya".to_string(), 
        "quince".to_string(), 
        "raspberry".to_string(), 
        "strawberry".to_string(), 
        "tangerine".to_string(),
        "watermelon".to_string(), 
        "zucchini".to_string()
    ];

    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}