// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

/// Pour que le code compile il faut soit enlever le point vigrule ou ajouter return avant num*num;
fn square(num: i32) -> i32 {
    num * num
}
