fn main11() {
    // let number = 3;
    let number = 7;

    if number < 5 {
        println!("condition was true"); // 条件は真でした
    } else {
        println!("condition was false"); // 条件は偽でした
    }

    if number != 0 {
        println!("number was something other than zero"); // 数値は0以外の何かです
    }
}

fn main22() {
    let number = 8;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn devide_zero(x: f32) -> f32{
    x / 0.0
}

fn main33() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    //let number = if condition { 5 } else { "six" };

    // numberの値は、{number}です
    println!("The value of number is: {number}");
}

fn main44() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // ループラベルを使用することで、breakやcontinueが適用されるループを指定することができます
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn main45() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");
}


fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() { // 発射！
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}