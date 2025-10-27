pub mod helpers;
pub mod closures;

use crate::closures::{test_closure, wingman_app};


fn main() {
    println!("Hello Rust , Olujimi Adebakin!");

    // test_func();
//     let myresult: String = helpers::name_helpers::get_full_name("jimi", "adebakin");
//      println!("Hello from: {}", myresult);

//      let my_num: i8 = helpers::api_helpers::get_age_plus_5(30);
//      println!("my_num is: {}", my_num);
    //  test_if();
    // test_while();
    // test_loop()
    // test_for();

    test_closure();
    wingman_app();
 }


// control flow
#[allow(dead_code)]
fn test_if(){
    let age_to_drive = 16u8;

    println!("Enter the person's age:");
    let my_input: &mut String = &mut String::from("");
    std::io::stdin().read_line( my_input).unwrap();

    let age = my_input.trim().parse::<u8>().unwrap();
    if age >= age_to_drive {
        println!("Issuing drivers license, because you are old enough");
    }
    else if age == 16 || age > 14 {
        println!("you are just on the verge of being old enough! Wait one more year.");
    }
    else {
        println!("Wait a bit longer, you aint old enough for a drivers license");
    }

    let drivers_license: bool = if age >= 16 {true} else {false};
}



#[allow(dead_code)]
fn test_while(){

    let age_to_drive = 16u8;

    let mut current_age = 0u8;

    while current_age < age_to_drive{
        println!("wating.......");

        current_age += 1;
    }
}

#[allow(dead_code)]
fn test_loop(){
    let mut x: i32 = 2;
    loop {
        println!("Hello from Rust ");

        if x > 6 {
            break;
        }

        x += 1;
    }
}


#[allow(dead_code)]
fn test_for(){
    let ages: [i32; 5] = [14, 18, 26, 35 ,41];
    let age_to_drive: i32 = 16i32;

       for value in ages {

        if value >= age_to_drive {
            println!("!You are not old enough to drive");
        }
        else {
            println!("You need to wait a little bit more ....")
        }
    }

}

#[allow(dead_code)]
fn test_func(){
    let x: f64 = 240.8;
    let y: u8 = x as u8 - 50;
    println!("y value is: {}", y);


    let mut i_am_old: bool = true;
     println!("Initial value: {}", i_am_old);
    i_am_old = false;
    print!("i_am_old is: {}", i_am_old);


    let mystr: char = 'A';
    println!("mystr is: {}", mystr);


    let mut first_name: &str = "Olujimi";
    print!("first_name is: {}", first_name);
    first_name = "Adebakin";
    println!(", last_name is: {}", first_name);


    // Tuple
    let my_tuple: (i32, f64, i8, &str, bool, u8) = (500, 6.4, 1, "adebakin", true, 40 as u8);
    println!("my_tuple is: {:?}", my_tuple);

    let ages: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("ages array is: {:?}", ages);


    // slice
     let ages: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("ages array is: {:?}", ages);

    let ages_slices:&[i32] = &ages[1..4];
     println!("ages_slice: {:?}", ages_slices);

    let new_ages: [u16; 3] = [ages[1] as u16, ages[2] as u16, ages[3] as u16];
    println!("new_ages: {:?}", new_ages);
    
}