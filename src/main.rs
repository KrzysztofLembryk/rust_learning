use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
 chapter_3_common_programming_concepts();
}

fn chapter_3_common_programming_concepts()
{
    // --------- VARIABLES ---------
    {
        // by default variables are IMMUTABLE
        let x = 5;
        // x = 69 won't compile

        // we can change them to be mutable by adding mut
        let mut y = 68;
        y = 69;

        // --------- CONSTANTS ---------
        // - you aren’t allowed to use mut with constants - always immutable
        // - type of constant needs to always be ANNOTATED
        const EXMPL_OF_CONST : i32 = 2137;


        // --------- SHADOWING ---------
        // - you can declare a new variable with the same name as a previous 
        //   variable
        let a = 65;
        let a = a + 4;
        println!("shadowed 'a' value {a}");
        {
            let a = a + 6900;
            println!("shadowed 'a' inside another scope {a}"); // will print 6969
        }

        println!("shadowed 'a' after the scope {a}"); // will print 69

        // Difference between mut and shadowing is that because we’re effectively 
        // creating a new variable when we use the let keyword again, we can change 
        // the type of the value but reuse the same name
        let a = "69xd";
        println!("{a}");
    }

    // --------- DATA TYPES ---------
    {
        // INTEGER OVERFLOW
        // Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors. When you’re compiling IN DEBUG MODE, Rust includes CHECKS FOR INTEGER OVERFLOW that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error

        // To explicitly handle integer overflows you need to use:
        // i.e. WRAPPING_ADD, CHECKED_ADD
        let mut sum: i8 = 0;
        for i in 1..127 {
            // we will just do modulo, and continue with calculating sum
            sum = sum.wrapping_add(i); 
            print!("{sum},");
            if i > 17
            {
                break;
            }
        }
        println!("\n##########################\n##################");
        sum = 0;

        for i in 1..127
        {
            print!("{sum} ");
            sum = match sum.checked_add(i)
            {
                Some(val) => val,
                None => {
                    println!("GOT OVERFLOW");
                    break;
                }
            };

        }

        // ------- TUPLES ------
        println!("###### TUPLES ######");
        // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a FIXED LENGTH: once declared, they cannot grow or shrink in size.
        let tupl: (i8, i32, f32) = (69, 6969, 21.37);

        // To get the individual values out of a tuple, we use PATTERN MATCHING to destructure a tuple value:
        let (x, y, z) = tupl;
        println!("x={x}, y={y}, z={z}");

        // or we can access by idx
        let six_nine = tupl.0;
        println!("idx.0={six_nine}");


        println!("######## ARRAYS ##########");
        // ---------- ARRRAYS ------------,
        // - have fixed nbr of elements, cannot grow
        // - to declare an array, you need to specify type and nbr of elements
        // i32 type of elems, 5 nbr of elems
        let arr1: [i32; 5] = [1, 2, 3, 4, 5];        

        // we can also declare it without supplying type and size
        let arr2 = [1, 2, 3];

        // accessing values
        println!("arr[0] = {}", arr2[0]);

        // if we access idx out of array scope we will get PANICK, since Rust
        // checks such thing and not allow us to access sth outside array
        

    }

    // --------- FUNCTIONS ----------
}

fn chapter_2_guessing_game()
{
    println!("Guess the number!");

    // 1..=100 is INCLUSIVE RANGE, meaning we include both 1 and 100
    // and create an array of numbers from 1 to 100
    // 1..100 is EXCLUSIVE RANGE, 100 is not included
    // gen_range method takes range as argument (start..=end)
    let secret_number = rand::thread_rng().gen_range(1..=50);

    // loop creates infty loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // We need to convert guess: String to u32 - secret_nbr type
        // We SHADOW previous guess with a new one.
        // - trim() --> eliminate any whitespace at the beginning and end
        // - parse() --> parse method on strings converts a string to another 
        //               type; we need to specify this type after :
        //               Because parse might fail, the parse method returns a 
        //               Result type. If it returns **Err** expect will catch 
        //               it, otherwise it will return **Ok** 
        // - expect() --> instance of RESULT has expect method so if Result is 
        //                Err expect will cause programme to crash
        // 
        // old version: our programme crashed when encountered not a number
        // let guess: u32 = guess.trim().parse().expect("Please type a number");

        // new version: we use pattern matching to handle Err case
        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Type a number!");
                continue;
            }
        };

        // Like in haskell, we have pattern matching , 
        // cmp returns Less, Greater of Equal value
        // More formally: A match expression is made up of arms. An arm consists of 
        // a pattern to match against, and the code that should be run if the value 
        // given to match fits that arm’s pattern
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You've guessed right!");
                break;
            }
        }
        println!("---------------------");
    }
}