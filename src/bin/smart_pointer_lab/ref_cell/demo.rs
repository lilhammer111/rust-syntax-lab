// 定义在外部库中的特征
pub trait Messenger {
    fn send(&self, msg: String);
}

// --------------------------
// 我们的代码中的数据结构和实现
struct MsgQueue<'a> {
    msg_cache: &'a mut Vec<String>,
}

impl<'b> Messenger for MsgQueue<'b> {
    fn send<>(&self, msg: String) {
        // 这是push()方法的签名：pub fn push(&mut self, value: T)
        // 签名中的“&mut self”说明push方法必须要一个可变引用来调用
        // 但这里是一个self是一个不可变引用，因此编译会报错：
        // self.msg_cache.push(msg)
        // ^^^^^^^^^^^^^^ `self` is a `&` reference,
        // so the data it refers to cannot be borrowed as mutable
        self.msg_cache.push(msg)
    }
}


fn main() {
    let mq = MsgQueue {
        msg_cache: &mut Vec::new(),
    };
    mq.send("hello, world".to_string());
}