trait Identifiable {
    fn id(&self) -> i32;
}

struct User {
    id: i32,
    name: String,
}

struct Order {
    id: i32,
    amount: i32,
}

impl Identifiable for User {
    fn id(&self) -> i32 {
        self.id
    }
}

impl Identifiable for Order {
    fn id(&self) -> i32 {
        self.id
    }
}

fn print_id<T: Identifiable>(item: &T) {
    println!("ID: {}", item.id());
}

fn main() {
    let user = User {
        id: 101,
        name: String::from("Ravi"),
    };

    let order = Order {
        id: 5001,
        amount: 2500,
    };

    println!("User: {}", user.name);
    println!("Order amount: {}", order.amount);

    print_id(&user);
    print_id(&order);
}