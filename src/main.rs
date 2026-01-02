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

    println!("{}", res);
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



 