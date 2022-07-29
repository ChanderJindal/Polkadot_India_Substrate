    
    fn main() {
        println!("{}, world", define_x()); 
    }
    
    fn define_x() -> &'static str{
        let x : &'static str = "hello";
      return x; 
    }