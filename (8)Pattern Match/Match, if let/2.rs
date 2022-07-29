
fn main() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = boolean as i32;

    assert_eq!(binary, 1);

    println!("Success!");
}
