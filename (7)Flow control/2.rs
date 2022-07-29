// Fix the errors
fn main() {
    let n = 5;

    let mut big_n = 0;
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

          big_n = 10 * n
        } else {
            println!(", and is a big number, halve the number");

          big_n =  n / 2.0 as i32 ;
        }

    println!("{} -> {}", n, big_n);
} 