// 这是使用原生指针的实现，相较于使用Cons(i32, Box<List>)的Box智能指针
// 这里使用原生指针并不会发生堆上数据，也即上一个List枚举对象的自动回收
// 所以需要我们手动回收内存，其实使用原生指针是直接操作内存的方式
// 在Rust中直接操作内存被视为一种不安全的行为，因此需要使用unsafe关键字
// 其次，在Rust中，引用类型的变量是可以借助借用检查来确保其生命周期要短于被引用数据的

mod type_demo;

enum List {
    Cons(i32, *mut List),
    Nil,
}

impl List {
    // 创建一个空的 List
    fn new() -> List {
        List::Nil
    }

    // 向 List 中添加元素
    unsafe fn prepend(head: *mut List, elem: i32) -> *mut List {
        let new_node = Box::new(List::Cons(elem, head));
        Box::into_raw(new_node)
    }

    // 安全地遍历 List（只读）
    unsafe fn iter(head: *const List) {
        let mut current = head;
        while let List::Cons(val, next) = &*current {
            println!("{}", val);
            current = &**next as *const List;
        }
    }

    // 手动释放 List
    unsafe fn drop(mut head: *mut List) {
        while let List::Cons(_, next) = &*head {
            if !next.is_null() {
                let _boxed_node = Box::from_raw(head);
                head = *next;
                // 当 boxed_node 离开作用域时，它所管理的内存会被自动释放
            } else {
                break;
            }
        }
    }
}

fn main() {
    unsafe {
        let mut list = List::new();
        let list_ptr = &mut list as *mut List;
        let list_ptr = List::prepend(list_ptr, 1);
        let list_ptr = List::prepend(list_ptr, 2);
        let list_ptr = List::prepend(list_ptr, 3);

        List::iter(list_ptr);
        List::drop(list_ptr);
    }

    println!("hello world")
}
