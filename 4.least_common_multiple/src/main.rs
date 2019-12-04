use std::io::{self, Write};
fn main() {
    print!("空白区切りで最小公倍数を求めたい２数を入力してください。 >> ");
    let _ = io::stdout().flush();
    let mut input = String::new();
    io::stdin().read_line(&mut input).
        expect("読み取りに失敗しました。");
    let list: Vec<&str> = input.split_whitespace().collect();
    let mut a:u32 = list[0].parse().expect("数字を入力してください。");
    let mut b:u32 = list[1].parse().expect("数字を入力してください。");
    let c:u32 = &a * &b;
    loop{
        if a > b {
            a = a - b;
        } else if b > a {
            b = b - a;
        } else {
            break;
        }
    }
    let result = c / b;

    println!("{} と{}の最大公約数は{} ,最小公倍数は{}です。", list[0], list[1],a,result);

}
