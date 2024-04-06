#  这里是✨Rust Syntax Lab✨

参考教材主要是[Rust Course](https://course.rs/)（rust圣经），
本项目主要是对书中没有详细展开或者新手学习时可能还会感到困惑的地方，以及作者自身联想到的问题进行一些语法实验与研究，
研究方式主要以更接近真实场景需求的代码例子的形式来辨析和深入理解Rust语法或者机制设计的功能和意义。

如果觉得不错的，给我点个⭐吧，谢谢🥰

# 🚀 项目计划

- [x] 项目启动 👍
- [x] 彻底弄懂生命周期标注 🥳
- [ ] 借用与所有权，Rust的基石 🫡
- [ ] 迭代器 😇
- [ ] 深入切片，有点意思 😊
- [ ] 智能指针，我的亲爹 🤕
- [ ] 动态数组 🤗
- [ ] Trait，应该不难吧 🤔

# 🔍 项目结构
```
src/  // rust源码
└── bin  // 二进制文件入口集合
    ├── borrow_lab  // 借用与所有权，Rust的基石
    ├── iterator_lab  // 迭代器
    ├── lifecycle_lab  // 彻底弄懂生命周期标注
    ├── slice_lab  // 深入切片
    ├── smart_pointer_lab  // 智能指针，我的亲爹~
    │   ├── box
    │   ├── cell
    │   ├── deref
    │   └── ref_cell
    ├── trait_lab  // Trait
    └── vec_lab  // 动态数组~
```

# 📖 使用方法
1. 先简单克隆一下项目吧~进入到一个你想放置该项目的dir然后执行这个命令吧
```bash
git clone https://github.com/lilhammer111/rust-syntax-lab.git
```
2. 选择想要了解的模块，比如lifecycle，打开文件`src/bin/lifecycle-lab/func_demo.rs`，按照注释中的解释一起动手操作，解开代码的注释，
运行看看吧，示例如下：
```rust
use std::env;

// 让我们从头开始，用每一个不同但相似的案例，来一步步分析和理解生命周期标注
// 在辨析这些案例之前，我们首先要明确一个“核心目标”：
// 即，被引用数据不能先于引用消亡，以下不同的表述都是同一意思
// （1）被引用数据的作用域应更长（2）被引用数据应该活得更久
// （3）引用的生命周期应该比被引用的生命周期短 （4）引用比被引用数据先“死”
// 这是理解生命周期的核心要诀
fn main() {
    let args: Vec<String> = env::args().collect();
    let string1 = args[1].clone();
    println!("The string1 is {}", string1);
    let result;
    {
        let string2 = args[2].clone();
        println!("The string2 is {}", string2);
        result = some_func(&string1, &string2);
    }
    println!("The result string is {}", result);
}

// 之后会被用到main函数
// fn main() {
//     let string1 = String::from("Here's the actual quoted data");
//     println!("The string1 is {}", string1);
//     let result;
//     {
//         let string2 = String::from("Insignificant data");
//         println!("The string2 is {}", string2);
//         result = some_func(&string1, &string2);
//     }
//     println!("The result string is {}", result);
// }

// 例一：
// 首先，咱们先不考虑生命周期标注的消除规则
// 因为这个函数的返回值的是引用，这个引用来自于x或者y其中一个，
// 因此我们得为返回值这个引用标注它和它指向的数据的关系，
// 这就是生命周期标注
// 我们看下面这种标注方式，并使用`cargo run --bin lifecycle hello world`命令来运行
// 编译器报了这样的错误：
// y
// ^ function was supposed to return data with lifetime `'a`
// but it is returning data with lifetime `'b`
// 这里意思是，我们在函数签名中为返回值标注了'a的生命周期，但是函数本身却有可能返回y，
// 也即'b的生命周期，但是编译器不知道'a和'b这两个生命周期孰长孰短
// 因此第一步我们得把'a和'b的关系给标注一下，至于'a'b的长短是否要按照实际调用上下文的情况来标注，
// 我们就暂不考虑，先随意标注两者长短关系，在后续的例子再继续探讨该问题

// 例一的some_func:
fn some_func<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
3. 执行`cargo run --bin func_demo arg1 arg2`来运行代码，或者如果，你是使用IDE，也可以直接点击main函数旁边的`run`按钮来执行
N.B.`cargo run --bin func_demo arg1 arg2`中的`func_demo`是`Cargo.toml`文件中[[bin]]下的name字段指定的名字！
```toml
# ...省略文件中的其他内容，具体可以打开自己看~

[[bin]]
name = "lifecycle"
path = "src/bin/lifecycle_lab/func_demo.rs"
```