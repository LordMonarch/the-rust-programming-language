fn main() {
    // infinite_loop();
    returning_values_from_loops();
    loop_labels();
    conditional_loops();
    looping_through_collection();
}

fn infinite_loop() {
    // Schleife die unendlich oder bis zum break läuft.
    loop {
        println!("again!");
    }
}

fn returning_values_from_loops() {
    // Das Statement hinter break wird aus der
    // Schleife zurückgegeben.
    // Dies kann verwendet werden um zu prüfen,
    // ob ein Thread abgeschlossen ist.
    // Ein return in der Schleife beendet die Funktion.
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    // break und continue wirken immer auf die
    // innere Schleife. Eine Schleife kann ein
    // Label bekommen, womit die beiden Befehle
    // auf sie ausgeführt werden können.
    // Loop label müssen mit einem ' beginnen
    let mut count = 0;
    'counting_up: loop {
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn conditional_loops() {
    let mut number = 3;

    // Die Schleife läuft so lange, wie die Bedingung true ist.
    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF");
}

fn looping_through_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Fehleranfällig, da der index zu einem Overflow führen kann.
    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    // Richtige Implementierung mit for!
    // Auch der Maschinencode ist effizienter als der von While.
    // For-loops sollte am meißten verwendet werden!
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    // Der Countdown als effizientere for-loop.
    // (1..4) erstellt eine Range einschließlich 1 und 3. 
    // rev() kehrt die Range um.
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
