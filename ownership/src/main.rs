fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    
    println!("{s}"); // this will print `hello, world!`
    
   //following code will raise an error to prevent double free after scope
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{s1}, world!");
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 = {s1}, s2 = {s2}");
}
//need to create a string object from literal to facilitate mutability
//rust calls drop(s) when string goes out of scope
