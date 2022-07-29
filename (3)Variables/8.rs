fn main(){
    fn8_mut();
    fn8_Shadowing();
}
    // Fix the error below with least amount of modification
    fn fn8_mut() {
        let (mut x, y) = (1, 2);
        x += 2;
    
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    
        println!("Success!");
    }
    
    //-----------------------------------------------------------------
    
    // Fix the error below with least amount of modification
    fn fn8_Shadowing() {
        let (x, y) = (1, 2);
        let x = x + 2;
    
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    
        println!("Success!");
    }
    