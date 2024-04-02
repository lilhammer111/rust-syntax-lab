mod closure_demo;

use std::collections::HashMap;

fn main() {
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];

    let zip_res = names.into_iter().zip(ages.into_iter());
    for zr in zip_res {
        println!("zip result: {:?}", zr)
    }

    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:?}", folks);
}