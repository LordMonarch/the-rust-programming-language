fn main() {
    variable_scope();
    string_type();
    ownership_and_functions();
    return_values_and_scope();
    return_tuple();
}

fn variable_scope() {
    // scope ist ein Bereich im Programm für den
    // ein Item valide ist.

    // s ist gültig für die ganze Methode
    let s = "hello";

    println!("{s}");

    {
        // t ist hier nicht valide, da es hier nicht
        // daklariert ist.

        // Von hier an ist t valide.
        let t = "hello";

        println!("{t}");

        // Die Scope ist vorbei, t ist nicht mehr valide.
    }
}

fn string_type() {
    // Literal (let a = "a") ist unveränderbar.
    // Ist nicht geeignet für dynamische Zwecke.

    // Dynamisch auf Heap.
    // Verwendet wenn zur Laufzeit die größe des Textes
    // unbekannt ist.
    // Er kann veränderbar sein, oder unveränderbar.
    let mut s = String::from("hallo");

    // push_str() appends a literal to a String
    s.push_str(", world!");

    println!("{s}");

    // Der Speicher wird wieder freigegeben, sobald die
    // Variable den scope verlässt.
    // Dafür wird die spezielle Methode drop() automatisch aufgerufen.

    // s2 ist ein Shallow copy von s1.
    // Gleichzeitig wird s1 unvalide gesetzt.
    // Die Operation heißt move.
    // Das verhindert, dass s1 und s2 beide versuchen nach dem
    // scope den Speicher freizugeben.
    // Nur s2 ist valide und wird den Speicher freigeben,
    // s1 ist nicht mehr valide.
    // Daher muss nach der zuweisung s2 verwendet werden.
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    // Wird der String Wert überschrieben, so ist der alte
    // Wert nicht mehr valide und wird vom Heap sofort gelöscht.
    // Erzeugt Warnung, dass der Wert nicht benutzt wird.
    let mut s10 = String::from("hello");
    s10 = String::from("ahoy");

    println!("{s10}, world!");

    // Heap Daten können Kopiert werden,
    // dass ist aber teuer!
    let a1 = String::from("hallo");
    let a2 = a1.clone();

    println!("s1 = {a1}, s2 = {a2}");

    // Stack Daten werden immer Kopiert, da es bei
    // ihnen keine zusatzkosten gibt. Für sie ist die
    // Größe bereits vorher bekannt. Es gibt keinen Painter.
    // Stack Daten sind alle Elementaren Daten und Tupel.
    // Arrays sind keine Stack Daten.
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

// Ownership and Functions
fn ownership_and_functions() {
    // s kommt in den scope
    let s = String::from("hello");

    // Der Wert von s geht in die Funktion (move).
    // Er ist in diesem Scope nicht mehr valide.
    take_ownership(s);

    // x kommt in den scope.
    let x = 5;

    // Weil i32 die Copy Eigenschaft hat,
    // gibt es kein move in die Funktion.
    // x kann weiterhin verwendet werden.
    makes_copy(x);

    // X verlässt denn scope, dann s.
    // Da s bereits den scope per move verlassen hat,
    // passiert nichts besonders mehr.
}

fn take_ownership(some_string: String) {
    // some_string kommt in den scope.
    println!("{some_string}");
    // some_string verlässt den scope.
    // drop() wird aufgerufen und der Speicher freigegeben.
}

fn makes_copy(some_integer: i32) {
    // some_integer kommt in den scope.
    println!("{some_integer}");
    // some_integer verlässt den scope.
    // Nichts besonderes passiert.
}

// Return Values and Scope
fn return_values_and_scope() {
    // gives_ownership move return Wert nach s1
    let s1 = gives_ownership();

    // s2 kommt in den scope
    let s2 = String::from("hello");

    // s2 move in die Funktion tages_and_gives_back,
    // und move in Variable s3
    let s3 = takes_and_gives_back(s2);

    // s3 geht aus dem scope und wird gelöscht.
    // s2 wurde move, also passiert nichts
    // s1 geht aus dem scioe und wird gelöscht.
}

fn gives_ownership() -> String {
    // gives_ownership move den Rückgabewert,
    // in die Funktion die sie aufruft.

    // some_string kommt in den scope.
    let some_string = String::from("yours");

    // some_string wird zurückgegeben und move in die
    // aufrufende Funktion.
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // Die Funktion nimmt einen String und gibt ihn zurück.

    // a_string kommt in den scope.
    // a_string wird zurückgegeben und move in die 
    // aufrufende Funktion.
    a_string
}

fn return_tuple() {
    // Es können auch Tuple zurückgegeben werden.
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("s2 = {s2}, len = {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    // len() gibt die Länge eines String zurück.
    let length = s.len();

    (s, length)
}