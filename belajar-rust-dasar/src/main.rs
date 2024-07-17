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
