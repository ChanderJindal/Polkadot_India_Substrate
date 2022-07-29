
// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");

    let ref r1 =  s;
    let ref r2 =  s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}

