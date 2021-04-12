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

    // bool
    {
        // Type annotation
        let _flag_a: bool = true;
        // Default type deduction
        let _flag_b = true;
    }

    // floats
    {
        let __a: f64 = 1.0; // double-precision float
        let _b: f32 = 1.0; // single-precision float

        let _c = 1.0; // f64
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

    // two more integer types
    {
        // the size it takes to reference any location in memory
        // (64 bit on x64 and 32 bit on x86)
        // (equals size_t, so use it to index some sort of collection)
        let _a: isize = 1;
        let _b: usize = 1;
    }

    // Tipp: usually just use the default integer type which is 32 bit and the fastest even on x64

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

    // lets print something to the console
    {
        let a = 400;
        let b = 20;

        println!("a + b = {}", a + b);

        println!("Binary literal: 0b10100111001 = {}", 0b10100111001);
        println!("Hexadecimal literal: 0x539 = {}", 0x539);
        println!("Octal literal: 0o2471 = {}", 0o2471);

        // Integer addition
        println!("1 + 2 = {}", 1u32 + 2);

        // Integer subtraction
        println!("1 - 2 = {}", 1i32 - 2);
        // this will not work with 1u32 because 1 - 2 = -1, but value is signed

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);

        println!("Emojis? But why? {}", '😻');
        println!("www {}", '草');
    }

    // convert types
    {
        // type annotations is necessary for the conversion
        let _guess: u32 = "42".parse().expect("Not a number!");
    }

    // compound types
    {
        // TUPLES (group together values (of different types))
        let _tup: (i32, f64, u8) = (500, 6.4, 1);

        // now let's access its values (this is called destructuring)
        let (x, y, z) = _tup;

        println!("My first tuple contains: {}, {}, {}", x, y, z);

        // you only want one element? no problem:
        let _y = _tup.1;

        println!("Only the second element in the tuple: {}", _y);

        // ARRAYS (of course every element must be of the same type)
        // obv. allocated on the stack, fixed length
        let _arr_a = [1, 2, 3, 4, 5];
        let _arr_b: [i8; 5] = [1, 2, 3, 4, 5];
        let _arr_c = [3; 5]; // [3, 3, 3, 3, 3]

        let _first = _arr_a[0];
    }

    // functions
    {
        function_a();
    }

    // How to compile in release mode:
    // $ rustc main.rs --release
    // This will cause integer overflow checks to be disabled and two complement's wrapping is used
    // This means, given a u8 (0 - 255):
    // 256 -> 0
    // 257 -> 1
    // Relying on this behaviour is considered an error
}


fn function_a( ) {
    let _y = {
        let _x = 3;
        _x + 1
    };

    println!("Result: {}", _y);
}