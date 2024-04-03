use std::collections::HashMap;

fn main() {
    let names = ["Demon", "Jojo"];
    let ages = [18, 18];

    let zip_res = names.into_iter().zip(ages.into_iter());
    for zr in zip_res {
        println!("zip result: {:?}", zr);
        println!("zip result name: {:?}", zr.0);
        println!("zip result age: {:?}", zr.1);
    }
}