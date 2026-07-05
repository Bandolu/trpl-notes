use std::io;

fn main001() {
    // let guess = "42".parse().expect("Not a number!");    // 数字ではありません！
    let guess: u32 = "42".parse().expect("Not a number!"); // 数字ではありません！
}

fn main002() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
                      //let _: () = x;
}

fn main003() {
    // addition
    // 足し算
    let sum = 5 + 10;

    println!("{sum}");

    // subtraction
    // 引き算
    let difference = 95.5 - 4.3;
    println!("{difference}");

    // multiplication
    // 掛け算
    let product = 4 * 30;
    println!("{product}");

    // division
    // 割り算
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
                            // 結果は-1
    println!("{quotient} {truncated}");

    // remainder
    // 余り
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation
                         // 明示的型注釈付きで
    println!("{remainder} {t} {f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
                       // 明示的型注釈付きで
    let heart_eyed_cat = '😻'; //ハート目の猫
    println!("{c} {z} {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let xyx: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = xyx.0;

    let six_point_four = xyx.1;

    let one = xyx.2;

    println!("{five_hundred} {six_point_four} {one}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first} {second}");
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    // 入力された値は数字ではありません

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // {index}番目の要素の値は{element}です
}
