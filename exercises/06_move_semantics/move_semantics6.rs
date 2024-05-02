// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

/// On modifie l'argument en rajoutant '&' à get_char qui borrow data et en l'enlevant à string_uppercase qui prendra possession de data
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
/// On rajoute le caractère '&' pour que la méthode borrow le String au lieu de prendre possession
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
/// On rajoute le caractère '&' pour que la méthode prenne la possession du String
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
