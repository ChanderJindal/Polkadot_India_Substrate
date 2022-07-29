// Fix errors without removing any line
fn main() {
    let s1 = String::from("hello,");
    let s2 = "world!";
    let mut s3 = s1.to_owned();
    s3.push_str(s2);    
    assert_eq!(s3,"hello,world!");
    println!("{}",s1);
}