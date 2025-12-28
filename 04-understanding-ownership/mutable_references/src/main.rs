// Referenzen sind unveränderlich. D.h. wenn Refrenzen übergeben werden sind sie immutable.
// Hier ist ein Code der Zeigt wie man die Referenz mutable macht.
fn main() {
    // Die Typ-Variable muss mutable sein.
    // Dann kann auch die Referenz mutable sein.
    // Es muss klar sein, das das Objekt veränderbar ist.
    let mut s = String::from("hello");

    change( &mut s);

    // Mehrere mutable Referenzen sind erlaubt.
    // Dafür benötigt man einen neuen scope.
    // Oder es darf keine Operation mit der Variable nach der definition
    // der zweiten Variable durchgeführt werden.
    {
        let r1 = &mut s;
        // r1 verlässt den scope, dann kann eine neue Referenz erstellt werden.
        println!("{r1}");
    }

    let r2 = &mut s;
    println!("{r2}");

    // Mehrere Unveränderliche Referenzen sind aber erlaubt.
    // Es darf keine mutable und immutable Referenzen auf die 
    // gleiche Variable zur gleichen Zeit geben.
    // Möglich ist es wenn die nach der definition nicht verwendet werden.
    let r3 = &s;
    let r4 = &s;
    println!("{r3}");
    println!("{r4}");
    // Würde den Code kaputt machen:
    // println!("{r2}");
}

// Die Funktion ist ein Statement.
// Es wird der Wert auf den die referenz zeigt geändert.
// WICHTIG: Es kann nur eine mutable Referenz auf ein Objekt geben!
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
