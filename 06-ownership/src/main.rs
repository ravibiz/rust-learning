fn take_ownership(name: String) {
    println!("Name: {}", name);
}

fn copy_value(number: i32) {
    println!("Number: {}", number);
}

fn clone_value(name: String) {
    let copied_name = name.clone();

    println!("Original: {}", name);
    println!("Cloned: {}", copied_name);
}

fn main() {
    // Move
    let name = String::from("Ravi");

    take_ownership(name);

    // `name` cannot be used here because ownership was moved.


    // Copy
    let number = 42;

    copy_value(number);

    println!("Number after function call: {}", number);


    // Clone
    let name = String::from("Rust");
    let cloned_name = name.clone();

    clone_value(name);

    println!("Cloned name: {}", cloned_name);
}