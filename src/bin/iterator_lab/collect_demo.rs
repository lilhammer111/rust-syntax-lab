use std::collections::HashMap;

fn main() {
    let names = ["Demon", "Jojo"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);
}