fn main001() {
    let mut x = 5;
    println!("The value of x is: {x}");     // xの値は{x}です
    x = 6;
    println!("The value of x is: {x}");
}

fn main002() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main(){
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{spaces}");

    println!("below may be error");
    let mut spaces = "   ";
    spaces = spaces.len();
    println!("{spaces}");
    
    
}