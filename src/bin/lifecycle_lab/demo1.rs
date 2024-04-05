use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string1 = args[1].clone();
    let result;
    {
        let string2 = args[2].clone();
        result = longest(&string1, &string2);
    }
    println!("The longest string is {}", result);
}


fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x.strip_prefix("hello").unwrap()
    }
}