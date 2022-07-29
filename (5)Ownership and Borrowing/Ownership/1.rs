fn main() {
    // Use as many approaches as you can to make it work
    let x : &'static str = "hello, world";
    let y = x;
    println!("{},{}",x,y);
}