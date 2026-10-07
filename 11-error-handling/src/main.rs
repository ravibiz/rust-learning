fn find_user_email(user_id: i32) -> Option<String> {
    if user_id == 1 {
        Some(String::from("ravi@example.com"))
    } else {
        None
    }
}

fn parse_age(input: &str) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(age) => Ok(age),
        Err(_) => Err(String::from("Invalid age")),
    }
}

fn main() {
    // Option
    match find_user_email(1) {
        Some(email) => println!("Email: {}", email),
        None => println!("Email not found"),
    }

    match find_user_email(2) {
        Some(email) => println!("Email: {}", email),
        None => println!("Email not found"),
    }

    // Result
    match parse_age("40") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(message) => println!("Error: {}", message),
    }

    match parse_age("abc") {
        Ok(age) => println!("Parsed age: {}", age),
        Err(message) => println!("Error: {}", message),
    }
}