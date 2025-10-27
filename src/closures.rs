pub fn test_closure(){
      let add  = |x: i8, y: u32| println!("Returning some numbers {} {}", x, y);
      add(7, 100);
}

pub fn wingman_app() {
    // The guy's confidence level
    let mut confidence = 0;

    // Define a closure that hypes him up
    let hype_up = || {
        println!("🔥 You got this bro! She’s totally into you!");
    };

    // Define a closure that checks if he's ready
    let mut is_ready  = |name: &str| -> bool {
        println!("Checking if {} has enough confidence...", name);
        confidence += 1; // captures & mutates outer variable
        confidence > 2
    };

    // Use the closures
    hype_up(); // prints hype message
    hype_up(); // prints again

    if is_ready("Jimi") {
        println!("✅ Jimi walks up and says hi with a smile 😎");
    } else {
        println!("😅 Still warming up… needs more hype.");
    }

    println!("Final confidence: {}", confidence);
}
