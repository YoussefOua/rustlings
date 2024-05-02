// strings4.rs
//
// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
//
// No hints this time!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}
/// "blue" est un &str
/// la méthode to_string() covertit le &str en String
/// String::from convertit un &str en String
/// la méthode to_owned() retourne un String
/// la méthode into() convertit le type en type d'entrée donc dans ce cas les deux possibilités sont correctes.
/// format! crée un String
/// &String::from("abc")[0..1] est un &str
/// la méthode trim() retourne un &str
/// la méthode replace() retourne un String
/// la méthode to_lowercase() retourne un String

fn main() {
    
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}