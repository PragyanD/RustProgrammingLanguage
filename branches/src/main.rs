fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
//let x = 2; if x {} does not compile. Explicit boolean required
//since if is an expression, can use on the RHS of a let statement
//eg. let condition = true;
//let number = if condition{ 5 } else { 6 };