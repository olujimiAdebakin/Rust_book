
#[warn(unused_variables)]
fn main() {
    println!("Hello Rust , Olujimi Adebakin!");
    test_func();
}

#[warn(unused_assignments)]
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
    
}