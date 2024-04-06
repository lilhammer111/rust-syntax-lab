use std::cell::Cell;

fn main() {
    // 当我们使用一个值的不可变引用时，我们是无法获取一个值的可变引用的（Rust借用规则）
    // 比如这样的代码:
    let some_str = String::from("Demon");
    let _the_immut = &some_str;
    // 再使用some_str的可变借用就无法通过编译了
    // IDE提示：Cannot borrow immutable local variable `some_str` as mutable
    // let _the_mut = &mut some_str;

    // 但是通过Cell<T>却可以实现可变和不可变引用同时存在
    let mom = Cell::new("qiaoyi");
    // 这相当于一个对字符串切片 Jojo 的不可变引用
    let fake_mom = &mom;
    // 这里相当于是可变引用，因为mom_immut_borrowed可以通过set方法修改被引用的值
    fake_mom.set("Jojo");
    // 这里mom的名字已经从qiaoyi被修改为Jojo了（可是你这样经过hammer的同意了吗？生气！）
    // fake_mom通过set方法修改了mom的行为，可能是mom不期望发生的，mom的修改权应该是由mom自己掌控
    // 因此一般场景下不应该使用Cell<T>，Cell<T>的合理使用场景见scenario.rs
    println!("mom's name is {} now", mom.get()); // mom's name is Jojo now

    // Cell<T>中的T必须实现Copy Trait
    // 因此下面的代码不能通过编译：
    // let str = Cell::new(String::from("hello"));
    // let s = &str;
    // s.set(String::from("world"));
    // println!("{}", str.get());
}