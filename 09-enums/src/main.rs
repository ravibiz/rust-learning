enum Status {
    Pending,
    Completed,
    Failed(String),
}

fn print_status(status: &Status) {
    match status {
        Status::Pending => {
            println!("Task is pending");
        }
        Status::Completed => {
            println!("Task is completed");
        }
        Status::Failed(message) => {
            println!("Task failed: {}", message);
        }
    }
}

fn main() {
    let pending = Status::Pending;
    let completed = Status::Completed;
    let failed = Status::Failed(String::from("Database unavailable"));

    print_status(&pending);
    print_status(&completed);
    print_status(&failed);
}