
// Konstanten mit const nicht let
// const mut ist verboten
// const kann ausßerhab vom scope definiert werden.
// Statische Berechnungen sind erlaubt.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; 

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    let y = 5;

    // Dieses y Überschattet das vorherige
    // Der neue Wert ist 6.
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }

    // Der Scope {} ist vorbei, der Wert der Variable ist wieder
    // der vor dem inner Scope.
    println!("The value of y is: {y}");

    // Erst ist spaces ein String.
    // Dann wird es ein Int.
    // let mut spaces ... ist nicht erlaubt.
    // let mut lässt sich nicht durch let überschreiben.
    let spaces = "    ";
    let spaces = spaces.len();

    println!("The value of spaces.len() is: {spaces}");
}
