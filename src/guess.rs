//! 这是一个猜谜游戏
use rand::Rng;
use std::io;
pub fn guess_number() {
    let result = rand::thread_rng().gen_range(1..101);
    println!("让我们开始猜数字游戏吧！");

    loop {
        println!("请输入你心中的数字！{}", result);
        let mut number = String::new();
        // 判断用户输入
        io::stdin().read_line(&mut number).unwrap();

        let guess_number: u32 = match number.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        // 作比较
        if result > guess_number {
            print!("你也太小了 官人~\n");
        } else if result < guess_number {
            println!("太大了官人~\n")
        } else {
            println!("官人，奴婢爱您~\n");
            break;
        }
    }
}
