fn main() {
    let s = gen_static_str();
    println!("{}", s);

    let s2 = gen_str();
    println!("{}", s2);



}

fn gen_static_str() -> &'static str {
    let mut s = String::new();
    s.push_str("hello, world");
    // s
    // 如果这里直接返回s会编译不通过，
    // 问题是函数入参和函数返回值都要求类型匹配，但偏偏在函数返回值的时候却不触发Autoderef，这是不是有些不公平呢？
    // 不这么做的核心原因：
    // 自动解引用可能涉及所有权的转移或生命周期的变化，特别是在涉及到引用的情况下。
    // 在返回值的上下文中，自动执行这样的操作可能导致难以预测的所有权或生命周期行为，进而影响代码的安全性。
    // 而函数入参就不会发生这种难以预测的所有权和生命周期行为，这就是这种“不公平”现象的背后逻辑。
    Box::leak(s.into_boxed_str())
    // Box::leak(Box::new(s.as_str()))
    // Box::leak(Box::new(s))
}

// 或者fn gen_str() -> &'static str 这样会更清晰
fn gen_str<'a>() -> &'a str {
    let s = "hello world";
    s
}
