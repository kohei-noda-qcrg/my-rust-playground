// https://www.youtube.com/watch?v=tw2WCjBTgRM&t=339s
fn main() {
    let mut x = 5;
    const CONSTANT: u32 = 100_000; // constant declaration, Must be annotated with a type
    println!("The value of CONSTANT is : {}", CONSTANT);
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    let y = 5;  // immutable variable declaration
    let y = y + 1;  // reassigning variable
    {
        // shadowing
        let y = y * 2;  // new variable declaration with the same name of the outer scope
        println!("The value of y in the inner scope is : {}", y); // 12
    }

    println!("The value of y is: {}", y); // 6

    // Redeclaring a variable with a different type is allowed
    let some_strings = "aaa";
    println!("The value of some_strings is: {}", some_strings);
    let some_strings = some_strings.len();
    println!("The value of some_strings is: {}", some_strings);

    // But reassigning a variable with mutable is not allowed
    // let mut some_strings = "aaa";
    // println!("The value of some_strings is: {}", some_strings);
    // some_strings = some_strings.len(); // error[E0308]: mismatched types
    // println!("The value of some_strings is: {}", some_strings);

}
