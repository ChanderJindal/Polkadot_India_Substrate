// Fix errors and panics to make it work
fn main() {
    //had to change the values
     let v1 = 51 + 8;
     let v2 = u8::checked_add(51, 8).unwrap();
    println!("{}, {}",v1,v2);
     //println!("{}",v2);
  }
  