// Person struct definition
// This struct represents a person with first and last names
// Both fields are Strings, meaning they own their text data
#[allow(dead_code)]
struct Person {
    first_name: String, // Person's first name
    last_name: String,  // Person's last name
}

// This function demonstrates various closure examples:
// - Basic closures with parameters
// - Closures that capture and use external variables
// - Closures that mutate external state
// - Different closure types (Fn, FnMut, FnOnce)
#[allow(dead_code)]
pub fn test_closure() {
    // Example 1: Simple closure that takes parameters and prints them
    // This closure doesn't capture any external variables - it's a pure function
    let add = |x: i8, y: u32| println!("Returning some numbers {} {}", x, y);
    add(7, 100); // Call the closure with actual values (no parameter names in call!)

    // Example 2: Closure that performs calculation and returns a value
    // Note: We're converting x from i32 to i8, which could cause overflow in real code
    let sum = |x: i32, y: i8| (x as i8) * y; // Closure returns the product

    let result = sum(3, 5); // Call closure and store result

    // Example 3: Closure that captures external variable 'result'
    // This is an Fn closure - it only reads the captured variable
    let print_sum = || println!("The Sum is: {}", result);
    print_sum(); // Prints the captured result

    // Example 4: Closure that mutates external state
    // Create a mutable Person instance
    let mut p1: Person = Person {
        first_name: "Jimi".to_string(),
        last_name: "Adebakin".to_string(),
    };

    // This closure captures p1 mutably (FnMut) and changes the last name
    let mut change_name = |new_last_name: &str| p1.last_name = new_last_name.to_string();

    change_name("Akanni"); // Call closure to change the last name

    println!("{}", p1.last_name); // Print the changed last name
}

// This function demonstrates a practical use case for closures:
// - Building confidence through multiple closure calls
// - Closures that capture and mutate external state
// - Real-world scenario of "wingman" encouraging someone
#[allow(dead_code)]
pub fn wingman_app() {
    // The guy's confidence level - stored as mutable so closures can update it
    let mut confidence = 0;

    // Closure 1: Pure Fn closure - just prints encouragement, no captured state
    // This can be called multiple times safely
    let hype_up = || {
        println!("🔥 You got this bro! She's totally into you!");
    };

    // Closure 2: FnMut closure - captures and MUTATES the confidence variable
    // Returns boolean indicating if confidence is high enough
    let mut is_ready = |name: &str| -> bool {
        println!("Checking if {} has enough confidence...", name);
        confidence += 1; // Mutates the captured confidence variable
        confidence > 2 // Returns true if confidence > 2
    };

    // Use the closures to build up confidence
    hype_up(); // First dose of encouragement
    hype_up(); // Second dose of encouragement

    // Check if he's ready to approach
    if is_ready("Jimi") {
        println!("✅ Jimi walks up and says hi with a smile 😎");
    } else {
        println!("😅 Still warming up… needs more hype.");
    }

    // Show the final confidence level after all operations
    println!("Final confidence: {}", confidence);
}
