pub fn test_match_int() {
    let my_age: u16 = 10;

    match my_age {
        10 => println!("You are just a kid"),
        15 => println!("You are a teenager"),
        20 => println!("You are a young adult"),
        30 => println!("You are an adult"),
        _ => println!("Age not categorized"),
    }

    let ch = 'b';

    match ch {
        'a'..='z' => println!("Lowercase letter"),
        'A'..='Z' => println!("Uppercase letter"),
        _ => println!("Not a letter"),
    }

    let number = 15;

    match number {
    1..=5 => println!("Between 1 and 5"),
    6..=10 => println!("Between 6 and 10"),
    _ => println!("Something else"),
}


let score = 87;

match score {
    90..=100 => println!("Grade: A"),
    80..=89 if score % 2 == 1 => println!("Grade: B (odd score!)"),
    80..=89 => println!("Grade: B"),
    70..=79 => println!("Grade: C"),
    _ => println!("Below average"),
}


let mood = "happy";

match mood {
      "happy" | "joyous" => println!("Vibes are very high"),
      "sad" | "broke" => println!("need some money"),
      _=> println!("Neutral vibes"),
}

let music = "Afroabeat";

match music {
     "Wizkid" | "Davido" => println!("Afrobeats Frauds 🔥"),
    "Burna Boy" | "Tuface" => println!("Legends in the game 💥"),
    other => println!("{} is doing great too 👏", other),
}

}
