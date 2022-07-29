    fn main(){
        fn1();
        fn11();
    }
    
    // Fix the error below with least amount of modification to the code
    fn fn1() {
        let x: i32; // Uninitialized but used, ERROR !
        let y: i32; // Uninitialized but also unused, only a Warning !
    
        //assert_eq!(x, 5);
        println!("Success!");
    }
    //This ^ one works, but I like this below
    
    fn fn11() {
        let x: i32 = 5;
        let y: i32 = 5; 
    
        assert_eq!(x, y);
        println!("Success!");
    }