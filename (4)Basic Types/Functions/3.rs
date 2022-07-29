// DON'T let `println!` works
fn main() {
    never_return_one();

    println!("Failed!");
}

fn never_return_one() -> ! {
use std::process;
process::exit(1);
}
fn never_return_two() -> ! {
assert!(false);
}
fn never_return_three() -> ! {
loop{
  continue;
}
}