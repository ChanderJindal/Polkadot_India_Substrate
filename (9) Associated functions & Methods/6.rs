
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color(&self) -> String{
     match *self {
       TrafficLightColor::Red => "red".to_string(),
       TrafficLightColor::Yellow => "yellow".to_string(),
       TrafficLightColor::Green => "green".to_string(),
       _ => "Not Found!".to_string()
    
}
      
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");
  //println!("{}",c.color());

    println!("{:?}",c);
  // to remove the & and * need to comment it ^
  // # Ownership and Borrow
}
