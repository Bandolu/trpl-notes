fn main11() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main22() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる。

    let r2 = &mut s;
    println!("{}", r2);

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main4() {
    {
        let mut s = String::from("hello");

        let r1 = &s; // 問題なし
        let r2 = &s; // 問題なし
                     //let r3 = &mut s; // 大問題！

        println!("{}, {}", r1, r2);
    }
    let mut s = String::from("hello!!!!");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    println!("{} and {}", r1, r2);
    // r1とr2はもうこれ以降使用されません

    let r3 = &mut s; // 問題なし
    println!("{}", r3);
}

fn main() {
    let no_ploblem = no_dangle();
}


fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
