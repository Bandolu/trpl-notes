fn main11() {
    println!("Hello, world!");

    another_function11();
}

fn another_function11() {
    println!("Another function.");  // 別の関数
}

fn main22() {
    another_function22(5);
}

fn another_function22(x: i32) {
    println!("The value of x is: {x}");   // xの値は{x}です
}

fn main33() {
    print_labeled_measurement(5, 'h');
    let y = {
        let x = 3;
        x + 1 //x + 1の行には文末にセミコロンがついていないことに気をつけてください
    };

    println!("The value of y is: {y}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}