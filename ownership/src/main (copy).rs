fn main11() {
    {
        // sは、ここでは有効ではない。まだ宣言されていない
        let s = "hello"; // sは、ここから有効になる

        // sで作業をする
    } // このスコープは終わり。もうsは有効ではない
    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()関数は、リテラルをStringに付け加える

    println!("{}", s); // これは`hello, world!`と出力する

    println!("replit is awesome!");
}

fn main22() {
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    //let s2 = s1;
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2)
}

fn main24() {
    let s = String::from("HOGE"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
    //println!("{}", s); // ... ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、
  // 何も特別なことは起こらない。

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

fn main255() {
  let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                      // ムーブする

  let s2 = String::from("hello");     // s2がスコープに入る

  let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                      // 戻り値もs3にムーブされる
  println!("s1 = {}, s3 = {}", s1, s3)
} // ここで、s3はスコープを抜け、ドロップされる。s2はムーブされているので、何も起きない。
// s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                           // 呼び出した関数にムーブする

  let some_string = String::from("yours"); // some_stringがスコープに入る

  some_string                              // some_stringが返され、呼び出し元関数に
                                           // ムーブされる
}

// この関数は、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

  a_string  // a_stringが返され、呼び出し元関数にムーブされる
}

fn main() {
  let s1 = String::from("hello");

  let (s2, len) = calculate_length(s1);

  //'{}'の長さは、{}です
  println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
  let length = s.len(); // len()メソッドは、Stringの長さを返します

  (s, length)
}