fn read_name(name: &String) {
    println!("Name: {}", name);
}

fn read_name_slice(name: &str) {
    println!("Name: {}", name);
}

fn update_name(name: &mut String) {
    name.push_str(" Kumar");
}

fn main() {
    // Immutable borrowing
    let name = String::from("Ravi");
    read_name(&name);
    println!("Name after borrowing: {}", name);

    // String slice borrowing
    read_name_slice(&name);

    // Mutable borrowing
    let mut name = String::from("Ravi");
    update_name(&mut name);
    println!("Name after mutable borrowing: {}", name);
}