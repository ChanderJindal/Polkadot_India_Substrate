
    // Fill the blanks
    enum Direction {
        East,
        West,
        North,
        South,
    }
    
    fn fn1_1() {
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South  => { // Matching South or North here
                println!("South or North");
            },
            Direction::West => println!("West"),
        };
    }
     
    