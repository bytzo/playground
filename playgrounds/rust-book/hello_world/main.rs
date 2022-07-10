// Rust files use the .rs file extension and should be named using snake_case.

// A function is defined using the fn keyword followed by its name, parameters,
// and body. The main function is a special function that acts as the program's
// entry point. It has no parameters and does not return anything.
fn main() {
    // Rust convention specifies four spaces for indentation. To format a Rust file
    // according to the Rust conventions, use the rustfmt command. For example, use
    // "rustfmt main.rs" to format this Rust file.

    // To print to the standard output, use the println macro. The ! after println
    // means it is a macro instead a function. The println macro accepts a string
    // to specify what is printed to the standard output. Strings can be created
    // using double quotes. Expressions in Rust always end with a semicolon.
    println!("Hello, world!");
}

// Rust files must be compiled before running. To compile a Rust file, use the
// rustc command. For example, use "rustc main.rs" to compile this Rust file.
