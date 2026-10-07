fn main() {
    let number = 10;

    if number > 5 {
        println!("Greater than 5");
    } else {
        println!("5 or less");
    }

    println!("For loop:");

    for i in 1..=5 {
        println!("{}", i);
    }

    println!("While loop:");

    let mut count = 1;

    while count <= 5 {
        println!("{}", count);
        count += 1;
    }

    println!("Loop:");

    let mut value = 1;

    loop {
        println!("{}", value);

        value += 1;

        if value > 3 {
            break;
        }
    }
}