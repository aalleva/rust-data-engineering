/*
    This example code counts the frequency of each number in the vector.
*/
use std::collections::HashMap;

fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();

    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }

    result
}

fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5];
    let result = logic(numbers);
    println!("The frequency of each number in the vector is {:?}", result);
}
