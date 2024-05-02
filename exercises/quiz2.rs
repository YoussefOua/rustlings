// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    /// L'entrée de la fonction est un vecteur de Tuple de 2 qui contient String et Command
    /// La fonction retourne un vecteur de String
    /// La fonction transformer vérifie la commande passée et traite le string en fonction de cette dernière
    /// Si la commande est trim la fonction applique trim() sur le String
    /// Si la commande est Uppercase la fonction transforme le String en lettres majuscules
    /// Si la commande est Append(n) la fonction rajoute n fois "bar" à String
    /// On utilisera un match pour gérer les differents cas
    /// Pour Append je me suis aidé d'une solution sur internet
    /// Il fallait mettre le match dans une variable et la push sur le vecteur
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let s = match command {
                Command::Uppercase => string.to_uppercase(),
                Command:: Trim => string.trim().to_string(),
                Command:: Append(n) => string.to_owned() + &"bar".repeat(*n),
            };
            output.push(s);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    /// On doit importer la fonction "transformer" du module "my_module" pour pouvoir l'utiliser
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
