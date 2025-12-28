fn main() {
    if_expression();
    multiple_conditions();
    using_if_in_let_statement();
}

fn if_expression() {
    let number = 3;

    // Nicht Boolean-Werte werden nicht automatsch
    // zu Boolean convertiert.
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn multiple_conditions() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_in_let_statement() {
    let condition = true;

    // Die Variable enthält den Wert nach der Auswertung des
    // if statements.
    // Beide möglichen Werte müsse vom gleichen Typ sein!
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
