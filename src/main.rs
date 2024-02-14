// fn main() {
//     let mut x= 0;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main() {
//     // addition
//     let sum = 5 + 10;
//     println!("The value of sum is: {sum}");

//     // subtraction
//     let difference = 95.5 - 4.3;
//     println!("The value of difference is: {difference}");

//     // multiplication
//     let product = 4 * 30;

//     println!("The value of product is: {product}");
    
//     // division
//     let quotient = 56.7 / 32.2;
//     println!("The value of quotient is: {quotient}");

//     let truncated = -5 / 3; // Results in -1 (Integer division truncates toward zero to the nearest integer.)
//     println!("The value of truncated is: {truncated}");

//     // remainder
//     let remainder = 43 % 5;
//     println!("The value of remainder is: {remainder}");
// }

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

    

//     let (x, y, z) = tup;

//     println!("The value of tup is: {x}, {y}, {z}");

//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("The value of tup is: {five_hundred}, {six_point_four}, {one}");
// }

// use std::io;

// fn main() {
//     let list = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = list[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main() {
//     let x = let y = 6;
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// fn main() {
//     let mut counter = 0;
//     // expression not statement
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// fn main() {
//     let mut count = 0;

//     'counting_up: loop {
//         println!("count = {count}");

//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");

//             if remaining == 9 {
//                 break;
//             }

//             if count == 2 {
//                 break 'counting_up;
//             }

//             remaining -=1;
//         }

//         count +=1;
//     }
// }

fn main() {
    let mut number = 3;

    while number !=0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}