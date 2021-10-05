enum Anything {
    K,
    A,
    B(u8),
}
fn match_let() {
    let a = match_enum(Anything::B(32));

    // 这里需要注意的是if let语法在进行匹配的时候，必须是需要匹配的表达式在等号的右侧。
    // 如果写反了的话就会抛出警告 let就是声明一个新的变量，与已经有的变量进行匹配
    if let 3 = a {
        print!("AAAA");
    } else {
        print!("error");
    }
}

fn match_enum(coin: Anything) -> u8 {
    match coin {
        Anything::A => 1,
        Anything::B(state) => state,
        Anything::K => 3,
    }
}
