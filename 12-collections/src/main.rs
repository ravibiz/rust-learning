use std::collections::HashMap;

fn main() {
    // Vec
    let mut response_times: Vec<i32> = Vec::new();

    response_times.push(80);
    response_times.push(120);
    response_times.push(95);

    println!("Response times:");

    for time in &response_times {
        println!("{}", time);
    }

    // Safe access with get()
    match response_times.get(1) {
        Some(time) => println!("Second response time: {}", time),
        None => println!("Response time not found"),
    }

    // HashMap
    let mut users: HashMap<i32, String> = HashMap::new();

    users.insert(1, String::from("Ravi"));
    users.insert(2, String::from("John"));

    match users.get(&1) {
        Some(name) => println!("User 1: {}", name),
        None => println!("User not found"),
    }
}