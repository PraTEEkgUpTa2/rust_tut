// fn main() {
//     println!("Hello, world!"); // invoking a macro

//     let res: u32 = sum(5,20);

//     let name: String = String::from("ALICE");

//     // vectors

//     let v: Vec<i32> = vec![1,2,3,4,5];
//     println!("{:?}", v);

//     // Immutable
//     // Rust is immutable by default
//     let x: i32 = 5;
//     // x = 6; // this will throw an error
//     // x += 1; // this will also throw an error

//     let mut y: i32 = 10;
//     y += 5; // this is valid


//     // Ownership in rust

//     let s1: String = String::from("hello");
//     let s2: String = s1; // s1 is moved to s2

//     // println!("{}", s1); // this will throw an error

//     // borrowing

//     let len = calculate_len(&s2); // passing reference of s2

//     println!("{}", res);

//     let r = Rect {
//         width: 10.0,
//         length: 20.0
//     };

//     println!("Width: {}, Length: {}", r.width, r.length);

//     // enum

//     let dir = Direction::North;


//     let shape1 = Shape::Square(10.0);
//     let shape2 = Shape::Circle(5.0);

//     println!("Area of shape1: {}", shape1.area());
     
// }

// fn steer(dir: Direction) {

//     // pattern matching
//     match dir {
//         Direction::North => println!("Steering North"),
//         Direction::South => println!("Steering South"),
//         _ => println!("Steering East or West"),
//     }
// }

// fn sum(a: u32, b: u32) -> u32{
//     return a + b;
// }

// // u32 -> unsigned 32 bit integer
// // i32 --> signed 32 bit integer

// // boolean function

// fn is_even(n: u32) -> bool{
//     return n%2 == 0;
// }


// // borrowing in rust

// fn calculate_len(s: &String) -> usize {
//     return s.len();
// }

// // Structs in rust


// struct Rect {
//     width: f32,
//     length: f32
// }

// impl Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.length;
//     }

//     // &self is a reference to instance of struct and make the function an member function
//     // fn new(width: f32, length: f32) -> Rect {
//     //     Rect { width, length }
//     // }


// }


// // enums in rust

// enum Direction {
//     North,
//     South,
//     East, 
//     West
// }

// // ENUM with values

// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32)
// }

// // enum can be implemented too 

// impl Shape {
//     fn area(&self) -> f32 {
//         match self {
//             Shape::Square(side) => side * side,
//             Shape::Circle(radius) => 3.14 * radius * radius,
//             Shape::Rectangle(width, length) => width * length,
//         }
//     }
// }

// // Option Enum & Result Enum are built-in enums in rust for handling null values and error handling respectively. 


// //



// use chrono::prelude::*;
// use dotenv::dotenv;
// use std::env;


// fn main() {
//     dotenv().ok();
//     let var = env::var("REDIS_ADD");

//     match var {
//         Ok(val) => println!("REDIS_ADD: {}", val),
//         Err(e) => println!("Couldn't read REDIS_ADD ({})", e),
//     }

//     let utc = Utc::now();
//     println!("Current UTC time: {}", utc);

//     // Generics in rust
//     // Generics allow you to write code that can work with different data types while ensuring type safety at compile time.

//     println!("Sum : {}", sum_with_bounds(5,10));

//     let r = Rect { 
//         width: 10.0,
//         length: 20.0
//     };

//     print_area_shape(r);

//     // println!("Area of rectangle: {}", r.area());
// }

// // fn sum<T>(a: T, b: T) -> T {
// //     return a + b;
// // } 
// // this will throw an error as + operator is not defined for generic type T for removing this error we can use trait bounds

// fn sum_with_bounds<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     return a + b;
// }


// // Generics with structs


// trait Shape {
//     fn area(&self) -> f32;

// }
// struct Rect {
//     width: f32,
//     length: f32
// }

// // impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
// //     fn area(&self) -> T {
// //         return self.width * self.length;
// //     }
// // }

// // trait



// // whoever implements this trait has to define area function

// impl Shape for Rect {
//     fn area(&self) -> f32 {
//         return self.width * self.length;
//     }
// }

// fn print_area_shape<T: Shape>(s: T) {
//     println!("Area: {}", s.area());
// }

// SERDE in rust

// use serde::{Serialize, Deserialize};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]

struct User {
    username: String,
    password: String,
}

fn main(){
    let u = User {
        username: String::from("alice"),
        password: String::from("password123"),
    };

    let mut v: Vec<u8> = Vec::new();

    //Convert struct to bytes and store in vector v
    let ans = u.serialize((&mut v));

    let user = User::try_from_slice(&v).unwrap();

    println!("Username: {}, Password: {}", user.username, user.password);

    // let serialized = serde_json::to_string(&u);

    // match serialized {
    //     Ok(s) => println!("Serialized User: {}", s),
    //     Err(e) => println!("Error serializing user: {}", e),
    // }

    // Lifetimes in rust

    let str1 = String::from("Hello, world!");

    let ans;
    {
        let str2 = String::from("Hello, Rust!");
        ans = longest(&str1, &str2);
        println!("Longest string: {}", ans);
    }

    
    // now this will throw an error as str2 goes out of scope and so prevent this we use lifetimes

    
}

fn longest<'a>(s1: &'a String, s2: &'a String) -> &'a String {
    if s1.len() > s2.len() {
        return s1;
    } else {
        return s2;
        
    }

    // this will always return lowest lifetime b/w s1 and s2
}