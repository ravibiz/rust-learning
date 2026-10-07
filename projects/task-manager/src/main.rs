use std::io;

struct Task {
    id: i32,
    title: String,
    status: TaskStatus,
}

enum TaskStatus {
    Pending,
    Completed,
}

impl Task {
    fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn is_completed(&self) -> bool {
        matches!(self.status, TaskStatus::Completed)
    }
}

trait Identifiable {
    fn id(&self) -> i32;
}

impl Identifiable for Task {
    fn id(&self) -> i32 {
        self.id
    }
}

fn find_by_id<T: Identifiable>(items: &[T], id: i32) -> Option<&T> {
    for item in items {
        if item.id() == id {
            return Some(item);
        }
    }

    None
}

fn find_task(tasks: &[Task], id: i32) {
    match find_by_id(tasks, id) {
        Some(task) => println!("Found task: {}", task.title),
        None => println!("Task not found"),
    }
}

fn complete_task(tasks: &mut [Task], id: i32) -> Result<(), String> {
    for task in tasks {
        if task.id == id {
            task.complete();
            return Ok(());
        }
    }

    Err(String::from("Task not found"))
}

fn add_task(tasks: &mut Vec<Task>, title: String, id: i32) {
    tasks.push(Task {
        id,
        title,
        status: TaskStatus::Pending,
    });
}

fn list_tasks(tasks: &[Task]) {
    for task in tasks {
        println!("Task {}: {}", task.id, task.title);
        println!("Completed: {}", task.is_completed());
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut next_id = 1;

    loop {
        println!();
        println!("Task Manager");
        println!("1. Add task");
        println!("2. List tasks");
        println!("3. Complete task");
        println!("4. Find task");
        println!("5. Exit");
        println!("Enter your choice:");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        match choice.trim() {
            "1" => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin()
                    .read_line(&mut title)
                    .expect("Failed to read input");
                let title = title.trim().to_string();
                add_task(&mut tasks, title, next_id);
                next_id += 1;
                println!("Task added successfully");
            }

            "2" => {
                list_tasks(&tasks);
            }

            "3" => {
                println!("Enter task ID to complete:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let id: i32 = match input.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid task ID");
                        continue;
                    }
                };
                match complete_task(&mut tasks, id) {
                    Ok(()) => println!("Task completed successfully"),
                    Err(message) => println!("Error: {}", message),
                }
            }

            "4" => {
                println!("Enter task ID to find:");
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");
                let id: i32 = match input.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid task ID");
                        continue;
                    }
                };
                find_task(&tasks, id);
            }

            "5" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid choice");
            }
        }
    }
}