#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}",i);
}

// 问题：
// 为什么我们都知道被结构体字段引用的数据应该比结构体活得更久，Rust还是让我们在上述结构体中为part字段标上'a的lifecycle?