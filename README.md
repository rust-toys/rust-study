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

## crate、package、Module

- package： cargo 的特性，它允许你构建、测试和分享 crate。（我们使用`cargo new xxx`就是创建一个包）
- crates：一个模块的树形结构（这一点很重要，它表明的是一个树形的结构，类似目录树），它形成了库和二进制项目
  - src/main.rs：二进制项目的`crate root`，即源文件入口。
  - src/lib.rs： 库项目的`crate root`
- Module： `mod`，允许我们控制作用域和路径的私有性。
  - `mod`后面如果跟着是个分号而不是代码段的时候，就会从 src 下去寻找同名的文件。例如`mod guess;`，就会去找`guess.rs` 文件，来引入该模块。
- path： 一个命名例如结构体、函数和模块等项的方式
  - 相对路径： `xxx::xxx` 。从当前模块开始，以 self、super 或当前模块的标识符开头，super 就相当于当前的父级目录，类似于我们在文件目录中的`../`（super 只能用在模块中，可以当做一个路径查找的方法）
  - 绝对路径：`crate::xxxx:xxx`（crate 就是一个模块的根级）从 crate 根开始，以 crate 名或者字面值 crate 开头

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}

fn main() {
    font_of_house::hosting::add_to_waitlist(); // 相对路径
    crate::font_of_house::hosting::add_to_waitlist(); // 绝对路径，crate是根目录
}

```

这就是该模块的树状结构：

![20211005124436](http://qiniu.liuqh.cn/blogImage/20211005124436.png)

## Vec<T>

创建 Vector

- `let v:Vec<i32> = Vec::new()`。这里需要注意必须显示的声明 Vec 的类型，并且`new`方法是没有参数的，只能通过`push`来添加元素
- `vec![1]` !就是 Vec 的一个宏

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v1 = &v[1];

    v.push(3);

    print!("{:?}", v1);
}
```

这段代码是会报错的：

```shell
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:5
  |
3 |     let v1 = &v[1];
  |               - immutable borrow occurs here
4 |
5 |     v.push(3);
  |     ^^^^^^^^^ mutable borrow occurs here
6 |
7 |     print!("{:?}", v1);
  |                    -- immutable borrow later used here
```

可以看到。我们在赋值 v1 的时候是不可变引用。然后在 v.push 的时候又产生了可变引用，就会报错，没有保证 v 的类型统一。

那么我们既然 v1 是获取的 v 的第一个元素，那么我们在末尾再添加元素的时候，会有什么影响呢？

其实这里牵扯到 Vec 在内存中的情况，Vec 在内存中是一个连续的内存空间，当我们在 push 元素的时候，如果这块连续的内存空间无法容纳新的元素，就会重新寻找另一个更大的连续内存空间，所有在 push 的时候，很可能这块内存空间已经移动了，原来的第一个元素的内存可能已经被释放了，所以就会造成 v1 访问不到（这就是悬垂指针？）

那么如果我们把代码改成下面的可以吗？

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let v1 = &mut v[1]; //变成可变的

    v.push(3);

    print!("{:?}", v1);
}
```

结果是**也不行**。因为可变引用在特定作用域下，同一时刻只能有一个可变的引用。

```shell
error[E0499]: cannot borrow `v` as mutable more than once at a time
 --> src/main.rs:5:5
  |
3 |     let v1 = &mut v[1];
  |                   - first mutable borrow occurs here
4 |
5 |     v.push(3);
  |     ^ second mutable borrow occurs here
6 |
7 |     print!("{:?}", v1);
  |                    -- first borrow later used here
```

## Vector 怎么存储不同类型的数据呢？

众所周知，rust 中的 vec 中元素的类型必须是相同的。
那么，有没有可能存储不一样的类型呢？

那必然是有的，就想下面这样：

```rust
#[derive(Debug)]
enum VecType {
    str(String),
    num(i32),
}

fn main() {
    let mut v = vec![VecType::str("字符串".to_string()), VecType::num(123)];

    print!("{:#?}", v)
}
```
