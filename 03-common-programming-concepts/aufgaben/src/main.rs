fn main() {
    let temperature = 20;
    println!("Celsius: {temperature}");

    let temperature = to_fahrenheit(temperature);
    println!("Fahrenheit: {temperature}");

    let temperature = to_celsius(temperature);
    println!("Celsius: {temperature}");

    println!();

    let n = 10;
    let fib = fibonacci(n);
    println!("Die {n}te Fibonnacci-Zahl ist {fib}");

    println!();

    let lyrics = twelve_days_for_christmas();
    println!("{lyrics}");
}

// Convert temperatures between Fahrenheit and Celsius
fn to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn to_fahrenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

// Generate the nth Fibonacci number.
fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    for _ in 1..n {
        let temp = a;
        a = b;
        b += temp;
    }
    a
}

// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
// taking advantage of the repetition in the song.
fn twelve_days_for_christmas() -> String {
    let mut lyrics = String::new();
    let mut count = 0;

    for _ in 1..13 {
        lyrics += "\nOn the first day of Christmas my true love sent to me";
        count += 1;

        if count >= 12 {
            lyrics += "\nTwelve drummers drumming,"
        }
        if count >= 11 {
            lyrics += "\nEleven pipers piping,"
        }
        if count >= 10 {
            lyrics += "\nTen lords a-leaping,"
        }
        if count >= 9 {
            lyrics += "\nNine ladies dancing,"
        }
        if count >= 8 {
            lyrics += "\nEight maids a-milking,"
        }
        if count >= 7 {
            lyrics += "\nSeven swans a-swimming,"
        }
        if count >= 6 {
            lyrics += "\nSix geese a-laying,"
        }
        if count >= 5 {
            lyrics += "\nFive gold rings,"
        }
        if count >= 4 {
            lyrics += "\nFour calling birds,"
        }
        if count >= 3 {
            lyrics += "\nThree French hens,"
        }
        if count >= 2 {
            lyrics += "\nTwo turtle doves,"
        }
        if count > 1 {
            lyrics += "\nAnd partridge in a pear tree."
        }
        if count == 1 {
            lyrics += "\nA partridge in a pear tree."
        }

        lyrics += "\n";
    }

    return lyrics;
}
