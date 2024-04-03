fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //  --> src/bin/borrow_lab/reborrow_demo:6:5
    //   |
    // 4 |     let first = &v[0];
    //   |                  - immutable borrow occurs here
    // 5 |
    // 6 |     v.push(6);
    //   |     ^^^^^^^^^ mutable borrow occurs here
    // 7 |
    // 8 |     println!("The first element is: {first}");
    //   |                                     ------- immutable borrow later used here

    // 这段代码如果执行会发生以上错误，但问题是为什么上述报错会说v.push()发生了一个借用行为呢？
    // 注意：我现在在讨论的是为什么是“借用”，而不是为什么是“可变借用”。

    // 原因是：
    // v.push()操作其实发生了一个自动再借用(reborrow)，在函数调用过程中，如果方法的签名要求的是引用比如&self
    // 或者&mut self，但是实际的调用者是一个值而非引用，这个时候就会发生自动再借用
    // 所以v.push(6)其实是(&mut v).push(6)，Rust编译器再用v的引用去调用这个方法。

    // 所以当我们将let first = &v[0];改为let first = v[0]时，这里只是一个复制v[0]的值给first，并没有引用，则不会报错啦
    // 但是要注意如果v的元素的类型并没有实现Copy Trait，那你并不能简单地把v[0]移出Vec，比如这样的Vec：
    // let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    // 这个时候let first = v[0];就报错了，因为你无法把Vec中的元素的所有权转移到动态数组外同时仍然期望使用这个Vec
    // 解决方法是let first = v[0].clone();
    v.push(6);

    println!("The first element is: {first}");
    println!("The vector is: {:?}", v);
}