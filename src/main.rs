use std::ops::Deref;

fn hello(s: &str) {
    println!("【 s 】==> {:?}", s);
}

fn main() {
    let my_box = MyBox::new(String::from("hello"));
    // 这里我们定义的方法的参数类型为&str
    // 但是我们传入的是 &MyBox<String>

    // 因为MyBox实现了Deref
    // 所以 deref &MyBox<String> => String;
    // 因为String也实现了Deref,返回的是一个切片类型
    // 所以 deref String => &str
    hello(&my_box);
}

// 实现自定义智能指针
struct MyBox<T>(T); // 定义MyBox的元组

// 为MyBox实现构造方法
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现Deref Trait
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
