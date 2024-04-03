fn main() {
    // 先看这三行代码，虽然在IDE（我用的RustRover）并没有报错，但是当我们执行代码时，我们会发现报了如下错误：
    //      let slice = a[1..3];
    //          ^^^^^ doesn't have a size known at compile-time
    // 原因分析：
    // 如果你注意到这句代码： let slice = a[1..3];,
    // 你会发现这与let slice = &a[1..3];有一个取引用的差别
    // 其实对于let slice = a[1..3];这里的slice类型是一个真正的切片类型，但切片是一个DST，因此会报错
    // 而对于let slice = &a[1..3];这时候的slice是一个切片引用类型，任何引用类型的大小都是固定的，这才没有报错
    let a = [1, 2, 3, 4, 5];
    // let slice = a[1..3];
    let slice = &a[1..3];
    println!("slice is {:?}", slice);

    // str也同理于上述分析，str的真正含义是：字符串切片类型，而&str也就是字符串字面值的类型才是字符串切片的引用类型
    let s = "hello";
    let hello = &s[..];
    println!("hello is {}", hello);

    let s = String::from("world");
    // 这里发生了自动解引用
    // 注意，非常重要的，取切片操作一直都是对&str而非str进行的，所以&s[0..5]这样的操作其实可以看成是&(s[0..5])
    // 所以必须要求这里的变量s是一个&str类型而非str类型，自动解引用可以吧String类型转成&str类型
    let world =&s[..3];
    println!("world is {}", world);
}