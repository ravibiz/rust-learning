fn print_value<T>(value: T) {
    println!("Value received");
}

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // Generic function
    print_value(42);
    print_value("Rust");

    // Generic function with a trait bound
    let larger_number = largest(10, 20);
    println!("Larger number: {}", larger_number);

    let larger_float = largest(10.5, 7.2);
    println!("Larger float: {}", larger_float);
}