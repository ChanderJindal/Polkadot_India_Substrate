
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn_one()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn_one() -> ! {
    use std::process;
process::exit(1);
}
fn never_return_fn_two() -> ! {
assert!(false);
}
fn never_return_fn_three() -> ! {
loop{
  continue;
}
}
