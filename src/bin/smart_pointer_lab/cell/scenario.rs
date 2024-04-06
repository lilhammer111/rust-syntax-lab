use std::cell::Cell;

// 这个例子可以说明cell存在的意义
// 另外内部可变性是指
// 即使程序的调用上下文中，调用者是不可变引用，
// 但是也可以通过内部封装了cell智能指针的方法，来实现内部封装的数据的改变
struct Config {
    // 假设我们有多个配置选项，这里只列举一个
    debug_mode: Cell<bool>,
}

impl Config {
    fn new(debug_mode: bool) -> Config {
        Config {
            debug_mode: Cell::new(debug_mode),
        }
    }

    fn is_debug_mode(&self) -> bool {
        self.debug_mode.get()
    }

    // 只有在特定条件下我们才会更改 debug 模式，不需要外部有 `&mut self`
    fn toggle_debug_mode(&self) {
        let current_mode = self.debug_mode.get();
        self.debug_mode.set(!current_mode);
    }
}

fn main() {
    let config = Config::new(false);

    // 在程序的不同部分读取配置，不需要可变引用
    println!("Initial debug mode: {}", config.is_debug_mode());

    // 在某个事件触发时更改配置，而不需要可变引用
    config.toggle_debug_mode();
    println!("Debug mode after toggle: {}", config.is_debug_mode());
}
