fn main() {
    let bird = Bird;
    let person = Human;
    bird.fly();
    person.fly();
    Flyable::fly();
    <Bird as Flyable>::fly();
}

trait Flyable {
    fn fly();
}

struct Human;

struct Bird;


impl Human {
    fn fly(&self) {
        println!("*the human is waving arms furiously*");
    }
}

impl Bird {
    fn fly(&self) {
        println!("*the bird is flying in normal*");
    }
}

impl Flyable for Bird {
    fn fly() {
        println!("birds can fly indeed")
    }
}

impl Flyable for Human {
    fn fly() {
        println!("human can not fly indeed")
    }
}