// fn main10() {
//     let mut x= 0;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main9() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// fn main8() {
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

// fn main7() {
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

// fn main6() {
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

// fn main5() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn main4() {
//     // does not work
//     // let x = let y = 6;
// }

// fn main3() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     return x + 1;
// }

// fn main1() {
//     loop {
//         println!("again!");
//     }
// }

// fn main2() {
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

// fn main11() {
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

// fn main412() {
//     let mut number = 3;

//     while number !=0 {
//         println!("{number}");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn main1523d() {
//     let a = [10, 20, 30, 40, 50];
//     let mut index = 0;

//     //while index < 5 // throws
//     while index < a.len() {
//         println!("the value is: {}", a[index]);

//         index +=1;
//     }
// }

// fn main() {
//     let a = [10,20,30,40,50];

//     for element in a {
//         println!("the value is {element}");
//     }
// }

// fn main() {
//     for number in (1..9).rev() {
//         println!("{number}!");
//     }

//     println!("LFG");
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);
//     println!("the first word is: {}", word);
    

//     s.clear(); // error!

// }

// fn main() {
//     let my_string = String::from("hello world");

//     // `first_word` works on slices of `String`s, whether partial or whole
//     let word = first_word(&my_string[0..6]);
//     let word = first_word(&my_string[..]);
//     // `first_word` also works on references to `String`s, which are equivalent
//     // to whole slices of `String`s
//     let word = first_word(&my_string);

//     let my_string_literal = "hello world";

//     // `first_word` works on slices of string literals, whether partial or whole
//     let word = first_word(&my_string_literal[0..6]);
//     let word = first_word(&my_string_literal[..]);

//     // Because string literals *are* string slices already,
//     // this works too, without the slice syntax!
//     let word = first_word(my_string_literal);
// }


///returns only index found space
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }


/// accept slice string and return sliced text string
// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             // return &s[..i]; wors as well
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             // return &s[..i]; wors as well
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     // let user1 = User {
//     //     active: true,
//     //     username: String::from("someusername123"),
//     //     email: String::from("someone@example.com"),
//     //     sign_in_count: 1,
//     // };

//     let user1 = build_user(String::from("someusername123"), String::from("someusername123"));

//     let user12 = user1.email;

//     println!("{user12}");

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

///tuples

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}


// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     if rect1.width() {
//         println!("The rectangle has a nonzero width; it is {}", rect1.width);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {}", rect1);
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:#?}", rect1);
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30*scale),
//         height: 50
//     };

//     dbg!(&rect1);
// }