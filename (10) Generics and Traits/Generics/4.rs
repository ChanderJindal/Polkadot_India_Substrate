
// Modify this struct to make the code work
struct Point<A,B> {
    x: A,
    y: B,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}
