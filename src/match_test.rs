
pub fn test_match_int(){

      let my_age: u16 = 10;

      match my_age {

            10 => println!("You are just a kid"),
            15 => println!("You are a teenager"),
            20 => println!("You are a young adult"),
            30 => println!("You are an adult"),
            _ => println!("Age not categorized"),
      }
}