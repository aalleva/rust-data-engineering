use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeSet;

fn main() {
    let fruits = vec![
        "apple",
        "banana",
        "orange",
        "grape",
        "kiwi",
        "mango",
        "pineapple",
        "strawberry",
    ];
    let amounts = [1, 3, 5, 7, 9, 11, 13, 15];

    let mut rng = thread_rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{} fruits: {:?}", amount, fruit_set);
    }
}
