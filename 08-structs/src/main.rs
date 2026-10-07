struct User {
    name: String,
    age: i32,
}

impl User {
    fn new(name: String, age: i32) -> User {
        User { name, age }
    }

    fn greet(&self) {
        println!("Hello, my name is {}", self.name);
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }

    fn print_details(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
    }
}

fn main() {
    let mut user = User::new(String::from("Ravi"), 40);

    user.greet();
    user.print_details();

    user.have_birthday();

    println!("After birthday:");
    user.print_details();
}