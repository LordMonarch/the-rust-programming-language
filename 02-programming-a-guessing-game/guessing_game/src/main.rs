use ::std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 1..=100 eine Range von 1 bis 100 inclusive 1 und 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    // loop Erschafft eine Endlosschleife.
    loop {
        // let definiert eine Variable
        // mut markiert die Variable als veränderbar
        // Ohne mut ist die Variable statisch (immutable).
        // String::new() führt die Funktion new() des Typ String aus.
        // Viele Typen besitzen die Funktion new() was ein neues Objext des Typs zurück gibt.
        let mut guess = String::new();

        // Erzeugt ein Objekt des Typ std::io:Stdin
        // &mut guess speichert String in Variable guess
        // String wird angefügt nicht Überschrieben
        // & ist eine Referenz auf guess
        // &mut ist eine veränderbare Referenz (mutable)
        // .expect() wird auf den Rückgabewert von read_line() angewandt.
        // Dieser Typ kann entweder die Variante OK oder Error haben.
        // Das expect() verhält sich dem Typ entsprechend.
        // Wird expect() nicht verwendet, gibt der Comiler eine Warnung.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parst den Sting in eine unsignd Integer.
        // Verwendet match um Fehler zu überspringen
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Match vergleicht die Variablen
        // Match hat verschiedenen Arme die bei bestimmten Bedingungen ausgeführt werden
        // Hier wird nur Text auf die Konsole geschrieben.
        // Mehrzeilige Anweisungen in {} einschließen.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
