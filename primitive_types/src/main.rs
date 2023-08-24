// https://www.youtube.com/watch?v=tw2WCjBTgRM&t=1154s

use std::io;

fn get_index_input(array: &[i32]) -> usize {
    // Get the array size
    let array_size = array.len();
    // Get input from user while the input is not a number
    let mut input = String::new();
    loop {
        println!("Please enter an array index.");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //  input is a natural number or 0.
        let _idx: usize = match input.trim().parse() {
            Ok(num) => {
                if num < array_size {
                    return num;
                } else {
                    println!("Please enter a number less than {}", array_size);
                    continue;
                }
            }
            Err(_) => continue,
        };
    }
}

fn main() {

    
    let x: usize = 5;
    let y: f64 = 1.5;
    // let z = x / y;  // error: mismatched types
    let z = x as f64 / y; // as is a cast operator
    println!("The value of z is : {}", z);

    let byte_u8_str = b"Hello World";
    println!("The value of byte_u8_str is : {:?}", byte_u8_str);

    let a = [1, 2, 3, 4, 5];

    let index = get_index_input(&a);
    println!("The value of index is : {}", index);
    let element = a[index];
    println!(
        "The value of the element at index {} is : {}",
        index, element
    );
}

