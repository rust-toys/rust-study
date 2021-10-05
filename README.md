# Rust 学习记录

> 详情查看 我的[个人博客](http://www.liuqh.cn)

## 借用和引用

借用的语法是`&`

```rust
let s = "hello,rust";
let s1 = &s;
```

默认情况下，rust 中的借用都是只读的，如果想要借用的为可变的需要使用`&mut`。

可变引用有一个很大的限制，就是在同一时间同一作用域下只能使用一个可变引用

```rust
fn main() {
    let mut s = "hello,rust";
    let s1 = &mut s;
    let s2 = &mut s;
    print!("{}{}", s1, s2)
}
```

这里会抛出一个错误`error[E0499]: cannot borrow 's' as mutable more than once at a time`。

> 其实在 rust 中借用和引用是同一个概念，但是与其他语言不同的是，其他语言中的引用其实就是共享所用权，对值有无差别的访问权限，可读可写。
> 但是 rust 默认情况下的引用是只读的

> 要清楚的认识到 rust 中设计所有权的用意，就是为了抛弃 GC（垃圾回收机制）来减轻运行时的负担，所有权的设计可以让 rust 在编译器通过检测规则来看使用者的代码设计是否符合其所有权的规则

## String 和 str 的区别

- `String`可以看做是由三个变量所组成的结构体：

  - 指向堆上连续内存的一个指针(红框)
  - 这块内存的总长度`capacity`（绿框）
  - 内存中已经使用的总大小`length`(蓝框)

- `String`类型是可以动态调整缓冲区容量的，所以有`push_str`等方法，可以动态扩充容量，其实`String`和`Vec<T>`类型的行为和特性在本质上一致，只不过`String`规定只能保存标准的`UTF-8`文本。

  ![20211003173322](http://qiniu.liuqh.cn/blogImage/20211003173322.png)

- `str`是字符串切片（slice），是对 String 的一种借用形式。他表示的是一个字符串的区间。并且它的长度是不固定的（unsized），它常用来表示一个引用，所以在使用中通常是`&str`，而不是`str`。字符串字面量默认就是`&str`

![20211003171141](http://qiniu.liuqh.cn/blogImage/20211003171141.png)
