// Characters (`char`)

fn main() {
    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: Analogous to the example before, declare a variable called `your_character`
    // below with your favorite character.
    // Try a letter, try a digit (in single quotes), try a special character, try a character
    // from a different language than your own, try an emoji 😉

    let your_character1 = 'Y';
    if your_character1.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character1.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character2 = '6';
    if your_character2.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character2.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character3 = '*';
    if your_character3.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character3.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character4 = 'ي';
    if your_character4.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character4.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character5 = '😉';
    if your_character5.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character5.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}