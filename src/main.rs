fn main() {
	// how to compile files in terminal: (user/.cargo/bin must be added to PATH on Windows)
	// $ rustc main.rs
	// $ ./main or $ main.exe

	// how to compile an entire project:
	// cargo is rust's build system and package manager
	// $ cargo new myProjectName
	// $ cd myProjectName
	// This also creates a .gitignore file (very cool!)
	// Now write your code and do:
	// $ cargo build // builds the project
	// $ cargo run // also builds the project

	// _ (underscore) in front of variable name will supress warning that the variable is not used

	// Type annotation
	let _flag_a: bool = true;
	// Default type deduction
	let _flag_b = true; 

	// floats
	{	
		let __a: f64 = 1.0;
		let _b: f32 = 1.0;
	}

	// signed integers
	{
		let _a: i128 = 1;
		let _b: i64 = 1;
		let _c: i32 = 1;
		let _d: i16 = 1;
		let _e: i8 = 1;

		let _f = 420; // i32
	}

	// unsigned integers
	{
		let _a: u128 = 1;
		let _b: u64 = 1;
		let _c: u32 = 1;
		let _d: u16 = 1;
		let _e: u8 = 1;

		let _f = 420; // u32
	}

	// char
	{
		let _a: char = 'a';
		let _b: char = 'b';
	}

	// immutable and mutable variables
	{
		let _a = 3.0; // f64 on my Windows/Linux x64 machine at least
		// a = 2.0; // illegal because default types are immutable (const)

		let mut _mut_a = 0.0;
		_mut_a = 1337.0;

		// mut_a = 'd'; // illegal: mismatched types
		// mut_a = 213; // illegal: mismatched types
	}

    println!("Hi");
}
