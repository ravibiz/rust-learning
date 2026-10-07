fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(value: i32) -> i32 {
    value * value
}

fn is_even(value: i32) -> bool {
    value % 2 == 0
}

fn main() {
    let sum = add(10, 20);
    let squared = square(5);

    println!("Sum: {}", sum);
    println!("Square: {}", squared);
    println!("Is 10 even? {}", is_even(10));
}