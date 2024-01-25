fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

//statements end with semicolon
//last experssion is used as return. explicit return uses return keyword
//x+1; would not compile