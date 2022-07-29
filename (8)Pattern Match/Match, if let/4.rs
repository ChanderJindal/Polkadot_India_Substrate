fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    // Fill the blank with `matches!` to make the code work
    for a in alphabets {
        assert!(matches!(a,'0'..='9' | 'A'..='Z' | 'a'..='z'))
    }

    println!("Success!");
} 