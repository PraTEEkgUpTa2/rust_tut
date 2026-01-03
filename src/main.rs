fn main() {
    println!("Hello, world!"); // invoking a macro

    let res: u32 = sum(5,20);

    let name: String = String::from("ALICE");

    // vectors

    let v: Vec<i32> = vec![1,2,3,4,5];
    println!("{:?}", v);

    // Immutable
    // Rust is immutable by default
    let x: i32 = 5;
    // x = 6; // this will throw an error
    // x += 1; // this will also throw an error

    let mut y: i32 = 10;
    y += 5; // this is valid


    // Ownership in rust

    let s1: String = String::from("hello");
    let s2: String = s1; // s1 is moved to s2

    // println!("{}", s1); // this will throw an error

    // borrowing

    let len = calculate_len(&s2); // passing reference of s2

    println!("{}", res);

    let r = Rect {
        width: 10.0,
        length: 20.0
    };

    println!("Width: {}, Length: {}", r.width, r.length);

    // enum

    let dir = Direction::North;


    let shape1 = Shape::Square(10.0);
    let shape2 = Shape::Circle(5.0);

    println!("Area of shape1: {}", shape1.area());
     
}

fn steer(dir: Direction) {

    // pattern matching
    match dir {
        Direction::North => println!("Steering North"),
        Direction::South => println!("Steering South"),
        _ => println!("Steering East or West"),
    }
}

fn sum(a: u32, b: u32) -> u32{
    return a + b;
}

// u32 -> unsigned 32 bit integer
// i32 --> signed 32 bit integer

// boolean function

fn is_even(n: u32) -> bool{
    return n%2 == 0;
}


// borrowing in rust

fn calculate_len(s: &String) -> usize {
    return s.len();
}

// Structs in rust


struct Rect {
    width: f32,
    length: f32
}

impl Rect {
    fn area(&self) -> f32 {
        return self.width * self.length;
    }

    // &self is a reference to instance of struct and make the function an member function
    // fn new(width: f32, length: f32) -> Rect {
    //     Rect { width, length }
    // }


}


// enums in rust

enum Direction {
    North,
    South,
    East, 
    West
}

// ENUM with values

enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32)
}

// enum can be implemented too 

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Square(side) => side * side,
            Shape::Circle(radius) => 3.14 * radius * radius,
            Shape::Rectangle(width, length) => width * length,
        }
    }
}

// Option Enum & Result Enum are built-in enums in rust for handling null values and error handling respectively. 