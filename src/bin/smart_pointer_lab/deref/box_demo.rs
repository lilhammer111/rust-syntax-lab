use std::ops::Deref;

fn main() {
    let s = MyBox::new(String::from("hello, world"));

    // 为什么let s1: &str = s;就会报类型不匹配呢？但是明明s_p的类型也不是&str但是却不会报类型不匹配的错误呢？
    // 原因是，s并不是一个引用类型，而是一个MyBox类型，只有引用类型才能自动解引用，因此let s1: &str = s;是会报错的
    //
    // let s1: &str = s;
    let s1: &str = &s;
    println!("s1 is {}", s1);
    // 而这里的s.to_string()的函数入参是&self（一个引用类型），但是s是一个非引用类型，却能成功调用to_string()
    // 原因是发生了自动再借用(reborrow)
    let s2: String = s.to_string();
    println!("s2 is {}", s2);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}


impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
