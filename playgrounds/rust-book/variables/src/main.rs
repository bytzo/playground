fn main() {
	// Variables in Rust are immutable by default, helping prevent logic errors when
	// the value of a variable changes unexpectedly. To make a variable mutable, use
	// the "mut" keyword.
	let mut x = 5; // "x" is declared as a mutable variable with a value of 5
	println!("The value of x is: {x}");
	x = 6; // The value of "x" can be changed to 6
	println!("The value of x is: {x}");

	// Use the "const" keyword to make a constant. A constant is a value that never
	// changes (like an immutable variable) but must also be bound to an expression
	// that is not computed at runtime. Constants must annotate their type and can
	// be declared in any scope. The naming convention for constants is
	// SCREAMING_SNAKE_CASE. For example, the constant below annotates its type as a
	// "u32" and is bound to a numeric value that is always the same (10800).
	const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

	// Variables can be shadowed by declaring another variable with the same name.
	// This is different from making a mutable variable because shadowing a variable
	// creates a new variable with the same name rather than modifying the original.
	let x = 5; // "x" is declared as an immutable variable with a value of 5
	let x = x + 1; // "x" is shadowed by "x" with a value of 6

	{
		// Shadowing a variable only takes place within the current scope. For example,
		// the shadowing of "x" below does not affect the print statement outside of
		// these curly braces.
		let x = x * 2;
		println!("The value of x in the inner scope is: {x}"); // Prints "The value of x in the inner scope is: 12"
	}

	println!("The value of x is: {x}"); // Prints "The value of x is: 6"

	// Because shadowing creates a new variable rather than modifying the original
	// one, the type of the new variable can be different from the original.
	let spaces = "   "; // "spaces" is a string
	let spaces = spaces.len(); // "spaces" is a number
}
