#![allow(dead_code)]        // 未使用の関数への警告をオフにする
#![allow(unused_variables)] // 未使用の変数への警告をオフにする

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main11() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
// wordの中身は、値5になる


    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効なのだ！
    println!("The 1st word is _{}_", word);
}

fn main22(){
  let s = String::from("This was a pen.");

  let hello = &s[0..4];
  let world = &s[5..8];
println!("{} {}", hello, world)
  
}

fn equal_content(){

    let s = String::from("hello");
    // len() を呼び出す。s の所有権は奪われない（&self だから）
    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..2];
    let slice = &s[..2];
    // だから、ここで s を使っても何の問題もない！
    println!("元の文字列も生きてるぞ: {}", s);
    println!("スライスの中身: {}", slice);
    println!("長さは: {}", len);
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s).to_string();

    s.clear(); // error! （エラー！）

    println!("the first word is: {}", word);

    //何かへの不変な参照がある時、さらに可変な参照を得ることはできないことを思い出してください。 clearはStringを切り詰める必要があるので、可変な参照を得る必要があります。 
    //clearの呼び出しの後のprintln!はword中の参照を使用するので、不変参照はその時点でもまだ有効でなくてはいけません。 Rustはclear中の可変参照とword中の不変参照が同時に存在することを認めないので、コンパイルが失敗します。
}