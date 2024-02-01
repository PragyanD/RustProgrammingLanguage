fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    
    println!("{s}"); // this will print `hello, world!`
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!");
}
//need to create a string object from literal to facilitate mutability
//rust calls drop(s) when string goes out of scope