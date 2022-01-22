use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(x, 5);
    assert_eq!(x, *y);
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
