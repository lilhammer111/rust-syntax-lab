fn main() {
    println!("Hello, world!");
}


struct Demon {}

impl Iterator for Demon {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}