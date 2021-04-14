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

    // How to compile in release mode:
    // $ rustc main.rs --release
    // This will cause integer overflow checks to be disabled and two complement's wrapping is used
    // This means, given a u8 (0 - 255):
    // 256 -> 0
    // 257 -> 1
    // Relying on this behaviour is considered an error

    // auto format project (just like with Clang Format) using
    // $ cargo fmt

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

        // assigning a mutable to a non mutable is possible
        // thus, this is not a true constant (see below for actual constants)
        let _b = _mut_a;

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
        // evaluations vs. expressions
        function_a();
        function_b(420, 1337);
        println!("Function c: {}", function_c());
        function_d(419);
    }

    // const
    {
        // replace let with const
        // always write const variables in upper case
        // you must always annotate the type
        // the main difference between normal variables is that they can
        // only be assigned constant expressions and not the output of a function
        // or anything else that is calculated during runtime
        const _MAX_POINT: u32 = 100_000;
    }

    // shadowing
    {
        // this is valid in rust and called shadowing
        // note how none of these were declared as mutable (mut)
        // you would get a compile-time error if you would not use let
        // this is useful if you have a case where a variable is only changed once
        // because you can just shadow it and it will continue being immutable again
        let x = 5;
        let x = x + 1;
        let x = x + 2; // x = 8

        println!("The value of x is: {}", x);

        // you can also use it to change a variable's type and reuse the same name
        let _spaces = "   ";
        let _spaces = _spaces.len();
    }

    // control structures
    {
        // IF - ELSE - ELSE IF
        let _number = 3;

        // not paranthesis are allowed but will trigger a warning
        if _number < 5 && _number >= 0 {
            println!("number smaller 5");
        } else if _number == 5 {
            println!("number equals 5");
        } else {
            println!("number greater 5");
        }

        let condition = true;
        let _number = if condition { 5 } else { 6 };

        // LOOPS
        {
            // endless loop (except for I break it directly)
            {
                loop {
                    break;
                }
            }

            // returning values from loops
            {
                let mut counter = 0;

                let _result = loop {
                    counter += 1;

                    if counter == 10 {
                        break counter * 2;
                    }
                };
            }

            // while
            {
                let a = [10, 20, 30, 40, 50];
                let mut index = 0;

                while index < 5 {
                    println!("the value is: {}", a[index]);

                    index += 1;
                }
            }

            // for (range approach)
            {
                // 1,2,3
                for number in 1..4 {
                    println!("{}!", number);
                }

                // 3,2,1
                for number in (1..4).rev() {
                    println!("{}!", number);
                }
            }

            // for (iterator approach)
            {
                let a = [10, 20, 30, 40, 50];

                for element in a.iter() {
                    println!("the value is: {}", element);
                }
            }
        }
    }

    // strings
    {
        let _s = String::from("hello");
    }
}

fn function_a() {
    let _y = {
        let _x = 3;
        // no semicolon because this is an evaluation
        // if it had one then it would be an expression
        // but you cannot assign an expression to an expression
        // only evaluation can be assigned
        _x + 1
    };

    println!("Result: {}", _y);
}

fn function_b(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_c() -> i32 {
    5
}

fn function_d(x: i32) -> i32 {
    x + 1 // this is an epxression (no semicolon)
          //x + 1; // this is a statement and not valid (expected i32 but found () )
}
