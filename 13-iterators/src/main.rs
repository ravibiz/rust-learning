fn main() {
    let response_times = vec![80, 120, 95, 200, 70];

    // Filter
    let slow_responses: Vec<i32> = response_times
        .iter()
        .filter(|time| **time > 100)
        .map(|time| *time)
        .collect();

    println!("Slow responses:");
    for time in &slow_responses {
        println!("{}", time);
    }

    // Map
    let doubled_times: Vec<i32> = response_times
        .iter()
        .map(|time| *time * 2)
        .collect();

    println!("Doubled response times:");
    for time in &doubled_times {
        println!("{}", time);
    }
}