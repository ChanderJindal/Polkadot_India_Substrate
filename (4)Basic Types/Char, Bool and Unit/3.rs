// Make println! work
fn main() {
    let f: bool = false;

    let t = true;
    if !f && t { // no unused variables either
        println!("Success!");
    }
} 