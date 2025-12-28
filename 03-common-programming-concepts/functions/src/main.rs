fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = 6;  // Statement x=y=6 nicht möglich

    println!("The value of statement y is: {y}");

    // Expression
    // Es darf kein ; am Ende stehen!
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of expression y is: {y}");

    let five = five();
    
    println!("The value of five is: {five}");

    let plus_one = plus_one(five);

    println!("The value of plus_one is: {plus_one}");
}

// Parameter sind die Variablen einer Funktion.
// Argumente sind die konkreten Werte einer Funktion.
fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

// Parameter müssen einen Typ haben.
// Die Parameter werden mit , separiert.
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Typ des Rückgabewertes anbeben mit ->
// Zurückgegeben wird der Ausdruck ohne ;
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}