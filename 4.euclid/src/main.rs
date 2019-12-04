use std::io::{self, Write};
fn main() {
    print!("最大公約数を求める２数をスペース区切りで入力してください >> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("読み取りに失敗しました。");
    let inputs: Vec<&str> = input.split_whitespace().collect();
    let mut a:u32 = inputs[0].parse().expect("数字を入力してください。");
    let mut b:u32 = inputs[1].parse().expect("数字を入力してください。");
    loop{
        if a > b {
            a = a - b;
        } else if b > a {
            b = b - a;
        } else {
            break;
        }
    }
    println!("最大公約数は{}",a);
}
