#![allow(dead_code)]        // 未使用の関数への警告をオフにする
#![allow(unused_variables)] // 未使用の変数への警告をオフにする


struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main11() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("email_old: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("email: {}", user1.email);

  // --snip--

  let user2 = User {
      active: user1.active,
      username: user1.username.clone(),
      email: String::from("another002@example.com"),
      sign_in_count: user1.sign_in_count,
  };

  // --snip--

  let user3 = User {
      email: String::from("another333@example.com"),
      ..user1
  };
  println!("email: {}", user3.email)
}

//省略記法
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
}
