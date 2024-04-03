use std::ops::Add;

fn main() {
    // 这里的a是一个Box<i32>类型
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
    // 但是可以使用*解引用出来，
    // 比较有意思的是，为什么Rust没有把let b = a + 1;中的a自动解引用为*a
    // 原因是自动解引用主要发生在以下上下文中：
    // 1. 函数和方法调用
    // 2. 类型匹配，如赋值操作 （这里虽然是b = a + 1，但并不被认为是一个严格的赋值操作的场景）
    // 3. 引用和借用
    let b = *a + 1;
    println!("b is {}", b);
    // 所以如果想让类型为Box<i32>的a在这种场景下自动解引用，一种可行的方案是为Box<i32>重载+操作符
    // define a note flag: Box<i32>+
    let c = MyBox(Box::new(5));
    let d = c + 1;
    println!("d is {:?}", d.0);


    let elems: Vec<Box<dyn Draw>> = vec![Box::new(Button { id: 1 }), Box::new(Select { id: 2 })];
    // 不可以下面这种写法，dyn Draw 是一个动态大小的类型（DST），意味着它在编译时大小不固定。
    // 因为 Rust 需要在编译时知道类型的大小，所以不能直接将 DST 放入 Vec 中。
    // 通常，我们通过使用指针（如 Box<dyn Draw>）来间接持有 dyn Draw，因为指针的大小是固定的。
    // let elems: Vec<dyn Draw> = vec![Button { id: 1 }, Select { id: 2 }];

    for e in elems {
        e.draw()
    }
}

// Box<i32>+:
// 以下是Box<i32>重载+操作符:
// 定义一个新类型，包装Box<i32>，为何要额外包裹一层，需见孤儿规则
struct MyBox(Box<i32>);

// 为 MyBox 实现 Add trait
impl Add<i32> for MyBox {
    type Output = MyBox;

    fn add(self, other: i32) -> MyBox {
        MyBox(Box::new(*self.0 + other))
    }
}


trait Draw {
    fn draw(&self);
}

struct Button {
    id: u32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("这是屏幕上第{}号按钮", self.id)
    }
}

struct Select {
    id: u32,
}

impl Draw for Select {
    fn draw(&self) {
        println!("这个选择框贼难用{}", self.id)
    }
}