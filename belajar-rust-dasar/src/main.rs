fn main() {
    println!("Hello, world!");

    println!("Hello, hafidz!");
}

#[test]
fn hello_test() {
    println!("Hello, test!");
}

#[test]
fn test_variable() {
    let name = "Hafidz Athaya";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Hafidz Athaya";
    println!("Hello {}", name);

    name = "Muhammad Hafidz Athaya";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Hafidz Athaya";
    println!("Hello {}", name);

    // name = 12345;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Hafidz Athaya";
    println!("Hello {}", name);

    let name = 12345;
    println!("Hello {}", name);

    let name = true;
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let age: i32 = 19;
    println!("{}", age);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("{}", a);

    let b: f32 = 10.5;
    println!("{}", b);
}

#[test]
fn number_convertion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    println!("{}", d);

    let e: i8 = d as i8; // Integer Overflow
    println!("{}", e);
}
