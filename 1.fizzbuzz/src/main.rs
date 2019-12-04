use std::io::{self, Write};
use std::time::{Instant};

fn main() {
    let mut input = String::new();
    print!("FizzBuzzの実行回数を入力してください。 > ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut input)
        .expect("読み込みに失敗しました。");

    let input: u32 = input.trim().parse()
        .expect("数値を入力してください。");

    let start = Instant::now();
    for i in 0..input {
        println!("{}", i);
        if i % 5 == 0 &&  i % 3 == 0 {
            println!("fizzbuzz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        }
    }
    let end = start.elapsed();
    println!("プログラム実行に{}秒かかりました。", end.as_secs())
}
