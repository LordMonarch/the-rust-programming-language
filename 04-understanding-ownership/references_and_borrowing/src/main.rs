// Beispiel wie eine Referenz an eine Funktion übergeben wird.
// Eine Referenz zeigt auf einen Validen Wert einer Variable.
// Unten wird eine Refernez wie ein Pointer übergeben.
// Die Funktion hat dann als Parameter die Referenz,
// und erhält kein ownerschip über die Variable.
// Dieser Vorgang heißt Borrowing.
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of \"{s1}\" is {len}");
}

fn calculate_length(s: &str) -> usize {
    // s ist eine Referenz zu einem String.
    s.len()
    // Hier verlässt s den scope. Da s aber nicht das Ownership besitzt,
    // wird der String nicht entfernt.
}
