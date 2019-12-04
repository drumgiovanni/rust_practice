use std::io::{self, Write};
fn main() {
    print!("計算式を半角スペース区切りで入力してください。ex) 3 + 2 > ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("読み取りに失敗しました");
    let list: Vec<&str> = input.split(" ").collect();
    // inputが３項と決まっている場合
    let val1: u32 = list[0].trim().parse().unwrap_or(0);
    let val2: u32 = list[2].trim().parse().unwrap_or(0);
    match list[1] {
        "+" => println!("{}", val1 + val2),
        "-" => println!("{}", val1 - val2),
        "*" => println!("{}", val1 * val2),
        "/" => println!("{}", val1 / val2),
        "%" => println!("{}", val1 % val2),
        _ => println!("四則演算子以外が入力されています。{}", list[1]),
    }
}
