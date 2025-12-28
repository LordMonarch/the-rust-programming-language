use std::io;

fn main() {

    // Der Compiler kann den Datentyp von parse() nicht selber ermitteln,
    // da mehrere Datentypen möglich sind (z.B. i32, u32).
    // Daher muss der Typ zwingend angegeben werden.
    let guess: u32 = "42".parse().expect("Not a number!");

    println!("The value of guess is: {guess}");

    // Standardwert für floats is 64-bit.
    // Floats sind immer signed.
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("\nFloating-Point Types");
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    // Addition
    let sum = 5 + 10;

    // Subtraktion
    let difference = 95.5 - 4.3;

    // Multiplikation
    let product = 4 * 30;

    // Division
    let quotient = 56.7 / 32.2;
    // Schneidet Nachkommateil ab.
    let trunicated = -5 / 3; 

    // remainder
    let remainder = 43 % 5;

    println!("\nNumeric Operations");
    println!("The value of sum is: {sum}");
    println!("The value of difference is: {difference}");
    println!("The value of product is: {product}");
    println!("The value of quotient is: {quotient}");
    println!("The value of trunicated is: {trunicated}");
    println!("The value of remainder is: {remainder}");

    let t = true;
    let f: bool = false;

    println!("\nThe Boolean Type");
    println!("The value of t is: {t}");
    println!("The value of f is: {f}");

    // Alle Chars sind mit ' nicht mit "
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("\nThe Character Type");
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // Tupel-Werte müssen nicht die selben Typen haben.
    // Tupel-Typ-Annotationen sind Optional.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    
    // Werte aus Tupel auslesen.
    let (x, y, z) = tup;

    // Werte aus Tupel auslesen.
    // Index beginnt mit 0.

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("\nThe Tuple Type");
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of five_hundred is: {five_hundred}");
    println!("The value of six_point_four is: {six_point_four}");
    println!("The value of one is: {one}");

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // Typannotation in [].
    // Erst Typ dann ; dann die Anzahl der Elemente.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Alternativ für Typ den ersten Wert direkt angeben.
    // Das Array enthält 5 mal den Wert 3.
    let a = [3; 5];

    let a = [1, 2, 3, 4, 5];
    // Index beginnt bei 0.
    let first = a[0];
    let second = a[1];

    println!("The Array Type");
    println!("The value of first is: {first}");
    println!("The value of second is: {second}");

    // Interaktives Beispiel für Index-Overflow
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

