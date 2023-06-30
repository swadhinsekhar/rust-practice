fn main() {
    /*
    // compiler throw error because enigma is not initialized.
    let enigma: i32;
    if true {
        enigma = 42;
    }
    println!("enigma: {}", enigma); //error
    */
    // to avoid above variable must initialised
    let enigma: i32;
    if true {
        enigma = 42;
    } else {
        enigma = 5;
    }
    println!("enigma: {}", enigma); //error
    println!("Hello, world!");
}
