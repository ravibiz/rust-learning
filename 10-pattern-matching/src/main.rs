enum Status {
    Pending,
    Completed,
    Failed(String),
}

fn describe_status(status: &Status) -> String {
    match status {
        Status::Pending => String::from("Task is waiting"),
        Status::Completed => String::from("Task is finished"),
        Status::Failed(message) => format!("Task failed: {}", message),
    }
}

fn main() {
    let statuses = vec![
        Status::Pending,
        Status::Completed,
        Status::Failed(String::from("Database unavailable")),
    ];

    for status in &statuses {
        let message = describe_status(status);
        println!("{}", message);
    }
}