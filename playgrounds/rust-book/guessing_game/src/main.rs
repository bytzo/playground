// Use the "use" keyword to bring the "io" module from the standard library into
// the scope of the program. This helps improve conciseness by allowing the "io"
// module to be used as "io" instead of "std::io". The io library helps with
// reading and writing to various inputs and outputs.
use std::io;

// Certain things from the standard library are automatically brought into the
// scope of every Rust program. The list of these things is called the prelude
// and helps make Rust programs more concise. Other preludes from the standard
// library (like "std::io::prelude") or external crates exist and can be used by
// bringing them into scope. The contents of the standard Rust prelude can be
// found at https://doc.rust-lang.org/stable/std/prelude/index.html.

// The "Rng" trait from the "rand" crate brings methods that random generators
// need into the scope of the program.
use rand::Rng;

// The "Ordering" type from the "cmp" module is an enum that has the variants
// "Less", "Greater", and "Equal".
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // The "thread_rng" function returns a random number generator that is local to
    // the current thread and seeded by the operating system. The "gen_range" method
    // from the "Rng" trait generates a random number in the specified range. A
    // range expression is used to specify a number between 1 and 100, inclusive.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Use the "loop" keyword to make an infinite loop. Without any "break"
    // statements, this will cause the program to repeat forever.
    loop {
        println!("Please input your guess.");

        // Use the "let" keyword to create a variable. Variables are immutable by
        // default and can be made mutable using the "mut" keyword. In the example
        // below, a new mutable variable called "guess" is bound to a new instance of a
        // "String" resulting from the "new" associated function. An associated function
        // is a function implemented on a type, which in this case is "String".
        let mut guess = String::new();

        // The "stdin" function returns a new instance of "Stdin" which represents the
        // system's standard input. The "read_line" method of "Stdin" reads characters
        // from the standard input until a newline character, then adds them to the
        // string passed as an argument. Using the "&mut" prefix indicates the argument
        // is a mutable reference since the original string needs to be modified by the
        // method. The "read_line" method also returns a "Result" enumeration, which is
        // a type with multiple states called variants. A "Result" has two variants:
        // "Ok", which contains the successfully generated value, or "Err", which
        // contains information about the error. The "expect" method of "Result" causes
        // the program to panic with the specified message if the result is "Err",
        // otherwise returns the value held by "Ok".
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // To ensure the "cmp" method works below, the type of "secret_number" and
        // "guess" must match. In the declaration of "guess" above, the type is inferred
        // to be a "String". However, "secret_number" is inferred to be an "i32", a
        // signed 32-bit number. To change the type of "guess", it can be shadowed by a
        // new variable with the same name but a different type. The "trim" method gets
        // rid of any whitespace before or after the string. This is needed to remove
        // the newline character at the end of the string created when the user pressed
        // return. The "parse" method then converts the string into another type. In
        // this case, "guess" is parsed into a "u32" since the new "guess" variable
        // annotates its type as a "u32" using a ":". By using the "u32" number type,
        // "secret_number" also becomes a "u32" since it is later compared with "guess".
        // Since the "parse" method returns a "Result" enum, a match expression can be
        // used to handle unparseable strings. In the first arm, the value held by "Ok"
        // is returned if the result is "Ok". In the second arm, the "continue"
        // statement is used to continue looping if the result is "Err". The underscore
        // is a catchall value, meaning that the value held by "Err" should be matched
        // no matter what.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // The "{}" is a placeholder and allows values to be included in strings.
        println!("You guessed: {guess}");

        // The "cmp" method compares two values and returns a variant of "Ordering". The
        // "match" keyword defines a match expression, which contains a number of arms
        // each with a pattern to match against and code to execute if the value matches
        // the pattern. In this scenario, the result of "cmp" is matched against the
        // variants of "Ordering" to inform the user of the status of their guess and to
        // exit if necessary.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Use the "break" statement to exit a loop.
                break;
            }
        }
    }
}
