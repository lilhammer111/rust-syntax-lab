fn main() {
    let mut v = vec![String::from("a"), String::from("b"), String::from("c")];
    let mut hello_world = String::from("hello world");
    // 这里的&mut v[1]是指可变借用了v中的第二个元素，也即对String::from("b")的可变借用
    // 而mut second是指这里的second变量（其实是一个引用类型），他是可变的
    // 因此可以写下second = &mut hello_world;这样的代码来修改second
    let mut second = &mut v[1];

    println!("The original second element is: {second}");
    second = &mut hello_world;
    println!("The second element changed is: {second}");


    // 而对于first变量而言，他并不是一个被mut修饰的变量因此他不具备可变性
    // 所以如果我们使用first = &hello_world这样的代码，无疑是编译不通过的
    // Cannot assign a new value to an immutable variable more than once [E0384]
    // 但是通过*first解引用取到了v的第0个元素，再去修改这个可变的String，那是完全没问题的
    let first = &mut v[0];
    println!("The first element is: {first}");

    println!("vec: {:?}", v);


}