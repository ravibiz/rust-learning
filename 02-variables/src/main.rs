fn main() {
    let name = "Ravi";
    let mut age = 40;

    println!("Name: {}", name);
    println!("Age: {}", age);

    age = 41;
    println!("Updated age: {}", age);

    let number = 10;
    let number = number + 5;
    println!("Shadowed number: {}", number);
}