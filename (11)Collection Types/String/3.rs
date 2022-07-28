
// Question: how many heap allocations are happened here ?
// Your answer: 2
fn main() {  
    // Create a String type based on `&str`
    // the type of string literals is `&str`
   let s: String = String::from("hello, world!");

   // create a slice point to String `s`
   let slice: &str = &s; // <- Here #1

   // create a String type based on the recently created slice
   let s: String = slice.to_string(); // <- Here #2

   assert_eq!(s, "hello, world!");

   println!("Success!")
}
