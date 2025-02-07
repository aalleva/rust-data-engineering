/*
This program creates a fruit salad by scrambling (shuffling) a list of fruit.
A vector is a growable array. It can grow or shrink in size and is one of the most
useful data structures in Rust. A vector is represented using the Vec<T> type.
*/

use rand::{rng, seq::SliceRandom};

fn main() {
    let mut fruit : Vec<&str> = vec![
        "apple", 
        "banana", 
        "cherry", 
        "durian", 
        "elderberry", 
        "fig", 
        "grape", 
        "honeydew", 
        "imbe", 
        "jackfruit", 
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
        "watermelon", 
        "xigua", 
        "yuzu", 
        "zucchini"
    ];

    let mut rng = rng();
    fruit.shuffle(&mut rng);

    println!("Fruit salad: ");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }

}
