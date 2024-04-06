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

// 例二的解释：
// 我们对例一中的函数签名稍加修改，只改动了生命周期声明中的'a，改为了'a: 'b
// 意思是： 给了'a一个约束，且这个约束是'a的生命周期至少比'b长
// 如果我们再使用`cargo run --bin lifecycle hello world`命令来运行
// 我们会发现他还是报了一样的错误！！！
// y
// ^ function was supposed to return data with lifetime `'a`
// but it is returning data with lifetime `'b`
// 这个道理也很简单，因为当我们声明了'a: 'b的约束时，
// 说明'a比'b活得久，在函数签名中，我们对返回值的生命周期标注也是'a
// 但是这个函数是有可能返回y的，并且y的生命周期是'b，
// 那这个时候返回值就可能返回一个lifecycle为'b的引用
// 这就和我们声明的函数返回值的生命周期是'a矛盾了，
// 注意这个所谓的“矛盾”是我们理解lifecycle的核心！
// 矛盾这一结论的推出不是理所当然的，而是因为对返回值这个引用我们标注了'a
// 就是在告诉编译器一件非常重要的事：
// 即，我作为这段程序的负责人，我可以很负责的告诉你（编译器），
// 我返回的引用--一定比'a活得要短--为什么是这样的语义?其实也很好理解，
// 因为我们的目的就是去保证--引用比被引用的生命周期短（核心目标）
// 因此，你作为编译器一定要好好帮我检查函数调用的上下文，
// 看看我函数的返回值（引用）是否真的要比实参（被引用者）先“死”，
// 但是，问题的重点来了，编译器一琢磨，心想：
// “你是说得跟真的一样，但我怎么就发现你的承诺（对返回值的生命周期标注）是句屁话呢？
// 这个函数不明明就有可能返回y吗？
// 那就算我检查上下文的时候，能确保返回值的生命周期（作用域）比'a要短，即便检查顺利通过了，（否则检查不通过报错）
// 但如果实际情况是返回y的引用，那返回值引用的就是y指向的数据，y指向的数据的生命周期又是'b，
// 这可比'a要短啊（按照'a: 'b的约束）
// 我在函数调用上下文检查时，只要你函数返回值的实际作用域比'a短，我就让你编译通过，
// 至于你是比'b短或者还是比'b长，我可不能保证，
// 因为你并没有给返回值标注为'b，因此我不做这样的检查呀。
// 简而言之，你给返回值标注什么生命周期，我就检查上下文是否满足实际返回值引用的生命周期比你标注得还短！
// 我只做这个事。

// 因此，程序员的这种标注（给返回值标注了'a），就有可能导致，
// 在实际情况下，函数确实返回了y指向的数据（生命周期为'b，比'a短），
// 而编译器只保证了返回值的作用域比'a短，并没有保证比'b短，（）
// 那就存在两种可能，要么比'b短，那么比'b长，这两种可能在实际情况都有可能发生（一句必须要强调的废话）
// 假如返回值实际的作用域比'b长的话，这个时候就会发生悬垂指针！
// --即，返回值引用的数据生命周期为'b，而返回值这个引用比被引用数据的生命周期'b还长
// 因此，编译器果断报错！
// 由此例，我们得出一个重要结论： 当函数入参有多个引用时，返回值的生命周期必须标注为引用生命周期最短的那个！

// 例二的some_func:
// fn some_func<'a:'b, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 例三的解释：
// 现在让我们尝试通过约束来修改'a和'b之间的关系，这里的关系是指'a,'b孰长孰短。
// 当我们把函数签名中的生命周期生命改为<'a, 'b: 'a>（其他不改），
// 'b: 'a这一约束的语义是，'a这个生命周期比'b更短
// 我们看函数返回值的生命周期标注！ Ok，标注的是'a，一个最短的生命周期！
// 一切都没问题了，让我们再使用`cargo run --bin lifecycle hello world`命令来运行（或者说编译吧）
// 编译器有一次无情地报错了！
// 他说：
// 16 |         println!("The result string is {}", string2);
// 17 |         result = some_func(&string1, &string2);
//    |                                      ^^^^^^^^ borrowed value does not live long enough
// 18 |     }
//    |     - `string2` dropped here while still borrowed
// 19 |     println!("The result string is {}", result);
//    |                                         ------ borrow later used here
// 其实这样的结果应该也是在我们预料之中的，让我们代入编译器的视角，来重现一下它当时的思考：
// 首先，编译器看到了some_func函数签名中的生命周期标注，它得到了如下信息：
// （1）根据'b: 'a，那'a比'b短
// （2）x指向的数据的生命周期是'a，那在函数调用的实际上下文中，x就是string1，他的作用域即下面这一段：
//     let string1 = args[1].clone();
//     println!("The string1 is {}", string1);
//     let result;
//     {
//         let string2 = args[2].clone();
//         println!("The string2 is {}", string2);
//         result = some_func(&string1, &string2);
//     }
//     println!("The result string is {}", result);
//  这一段我先标注为'a
// （3）y指向的数据的生命周期是'b，在函数调用的实际上下文中就是string2，它的作用域是：
//    {
//         let string2 = args[2].clone();
//         println!("The string2 is {}", string2);
//         result = some_func(&string1, &string2);
//     }
//  这一段我先标注为'b
// （4）最后返回值的生命周期标注是'a，这个语义和函数入参的生命周期标注可是完全不同的，他表示的是一种保证，
// 即返回值的实际生命周期应该应该比刚刚在上下文中标注的作用域'a要小
// 那返回值的实际作用域是什么呢？见下面：
//     let result;
//     {
//         let string2 = args[2].clone();
//         println!("The string2 is {}", string2);
//         result = some_func(&string1, &string2);
//     }
//     println!("The result string is {}", result);
// （5）函数返回值的作用域实际确实比'a要小，但是问题是在我们的约束中，我们声明了'b比'a还小（'a: 'b约束），
// 虽然实际情况并非如此（这个细节后续可以继续分析），那函数返回值的作用域得'b也小，
// 那答案就出来了！经过编译器的这五步分析，他发现函数实际的作用域是比'b长的，
// 不满足程序员对some_func返回值生命周期的要求--返回值生命周期应该比'a，'b都短--因此，我要赶快报错告诉程序员这件事！！！
// 结论：
// 从我们错误地标注了'a和'b的实际长短关系可以看出，生命周期标注本质只是程序员自己的一个声明，
// 而目的就是让编译器按照我们的需求去做“悬空引用”的检查，确保在我们的需求下，不会存在问题就行
// 这句话得好好推敲，因为他的等价意思是，
// 即使程序本身是没问题，但是程序员的标注下有问题，那你编译器就给我报错！
// 最精彩的一幕可能要到来了
// 让我们把原先的main函数注释，打开那个“之后会被用到main函数”
// 从上下文中，我们知道result实际引用的就是string1，而string1的作用域就是比result长，
// 我们知道这绝不可能发生悬垂指针的情况！
// 但是编译器在做生命周期检查的时候，并不会“想这么多”，而是，“我就按你的需求做检查”，
// 你给我了x,y的生命周期为'a,'b这些条件，那我就把x,y实际的作用域标为'a，'b,
// 并且让我检查result的作用域是否比'a,'b短，那我就只检查这一点，
// 我也不会去考虑你对'a 'b熟长熟短的标注跟实际情况是否相符！！！
// 在生命周期的检查上，编译器似乎表现的没有那么尽心尽力，就像一个只是机械地去执行上级发布的命令的机器
// 并不会去思考问题的实质，
// 这种设计的本质原因还是在于“生命周期检查”本身是一件极其复杂的事，我们现在举例的这些情况可能只是冰山一角
// 于是，相比去设计复杂的算法还有可能出错，倒不如把这件事交给“智慧”的程序员，让他们自己对自己的代码负责！
// 而“我”--编译器--就老老实实按照我程序员老板的要求去为他做检查就行，不必去思考老板的决策对错与否。
// 这就是“生命周期检查”的实质！！！

// 例三的some_func:
// fn some_func<'a, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// 总结：
// 相信经过上面几个案例的分析，我们也差不多理解了生命周期标注到底是在干嘛，他的目的和意义在哪？以及为什么要这么做！
// 当我们在回头看过来这样一段main函数里的some_func函数调用上下文和some_func这个函数
// 你会如何去为some_func去做生命周期标注呢？
// fn some_func(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// 还是说，
// 从另一个视角，对some_func进行生命周期标注的过程恰恰让我们意识到some_func函数的调用上下文存在悬空指针的问题
// 我们难道不可以修改函数调用上下文吗？就像这样：
// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let string1 = args[1].clone();
//     println!("The string1 is {}", string1);
//     let result;
//     let string2 = args[2].clone();
//     println!("The string2 is {}", string2);
//     result = some_func(&string1, &string2);
//     println!("The result string is {}", result);
// }
// fn some_func<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// cargo run --bin lifecycle hello lifecycle