use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    chapter_4_OWNERSHIP();
}

fn chapter_4_OWNERSHIP()
{
    let is_ownership = true;
    let is_string_type = true;

    if is_ownership {
        println!("############# OWNERSHIP #############");

        // OWNERSHIP RULES:
        // -- Each value in Rust has an owner.
        // -- THERE CAN ONLY BE ONE OWNER AT A TIME.
        // -- When the owner goes out of scope, the value will be dropped.

        // The variable s  is valid from the point at which it’s declared UNTIL THE END OF THE CURRENT SCOPE.

        { // we create a local scope
            let s = "some variable";
            println!("s value inside scope: {s}");
        } 
        // the scope is over, so s is NO LONGER VALID
        // this will cause compiler error
        // println!("value of s: {s}");
        // So:
        // - When s comes into scope, it is valid.
        // - It remains valid until it goes out of scope.

    }

    if is_string_type {
        println!("############# STRING TYPE #############");
        // We want to look at data that is stored on the heap and explore how 
        // Rust knows when to clean up that data, String type is a great example

        // string literal type, a string value is hardcoded into our program
        // we cannot modify it
        let str_literal = "string literal value";
        println!("str literal: {}", str_literal);

        // String type: manages data allocated on the heap and as such is able 
        // to store an unknown amount of text 

        // creating a mutable string
        let mut mutable_str = String::from("allocated data string");

        mutable_str.push_str(", newly pushed string");

        println!("mutable str: {mutable_str}");

        // WHY we cannot modify string literal? We know the contents at compile time, so the text is hardcoded directly into the final executable
        // With String type we allocate memoty on heap, unknown at compile time
        // This means:
        // -- The memory must be requested from the memory allocator at runtime.
        //     ---> this is done by us using String::from
        // -- We need a way of returning this memory to the allocator when we’re done with our String.
        //     ---> this is where ownership comes

        // Changing ownership etc
        println!("########### Variables and Data Interacting with Move ###########");

        // since integers are simple data types with known FIXED size, thus
        // they will be copied and pushed on stack  
        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");

        // HOWEVER this is not the case with below example
        let s1 = String::from("some string");
        let s2 = s1;

        // s2 will not have a copy of s1, but s2 will BE THE NEW AND ONLY OWNER
        // of s1 string data, thus after s2 = s1 line, s1 is NO LONGER VALID
        // and does not point to our String data 

    }

}

fn chapter_3_common_programming_concepts()
{
    let is_variables = true;
    let is_data_types = true;
    let is_functions = true;
    let is_conditions = true;
    let is_loops = true;
    // --------- VARIABLES ---------
    
    if is_variables {
        println!("############# VARIABLES #############");
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
    if is_data_types {
        println!("############# DATA TYPES #############");
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
    if is_functions {
        println!("############# FUNCTIONS #############");
        // Rust doesn’t care where you define your functions, can be before or
        // after the main function, it still will be visible
        // STATEMENTS AND EXPRESSIONS
        // -> Statements are instructions that perform some action and do not return a value.
        // -> Expressions evaluate to a resultant value.

        let y = 69; // this is statement, does not return value

        // let x = (let y = 6); this will not compile since statements do not
        // return values
        // In C this will compile int i = j = 6; but not in Rust
        
        // Expressions can be part of statement
        let y = {
            let x = 3;
            x + 1
        };

        // function declaration:
        fn plus_one(x: i32) -> i32 {
            x + 1 // this is an expression, thus it evaluates, thus we dont need ;
        }

        // But if we did x+1;, we would get an error, since x+1; is no longer 
        // an expression, but a statement
    }

    // --------- CONDITIONS ----------
    if is_conditions {
        println!("############# CONDITIONS #############");
        // conditions in IF must be a BOOL
        let val = 3;
        if  val > 0 
        {
            println!("val: {val} > 0");
        }

        // This will not compile since VAL IS NOT BOOl
        // if val {
        //     println!("Won't compile since statement in if is not bool but 3")
        // }

        // BECAUSE IF IS AN EXPRESSION, we can use it on the right side of a let statement to assign the outcome to a variable
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("Using if in rhs of let succeded,value of number is:{number}");
    }

    // ---------- LOOPS -----------
    if is_loops {
        println!("############# LOOPS #############");
        // we have three loop key-words:
        // - loop
        // - for
        // - while
        
        // ###### LOOP ######
        // -> will loop forever
        // One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to PASS THE RESULT OF THAT OPERATION OUT OF THE LOOP to the rest of your code. To do this, you can ADD THE VALUE YOU WANT RETURNED AFTER THE BREAK expression you use to stop the loop; that value will be returned out of the loop

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter > 10 
            {
                break counter * counter
            }
        };
        println!("result of breaking from loop {result}");

        // !!!!!!!!!!!  LOOP LABELS  !!!!!!!!!!!!!!
        let mut count = 0;
        
        println!("--LOOP LABELS--");
        'main_loop: loop {
            println!("main lloopp count: {count}");

            let mut nbr_of_loops = 3;

            loop {
                println!("nbr_of_loops: {nbr_of_loops}");

                if nbr_of_loops < 3 
                {
                    break;
                }
                if count == 1
                {
                    println!("breaking main loop from inner loop");
                    break 'main_loop;
                }
                nbr_of_loops -= 1;
            }
            count += 1;
        }
        println!("End of main loop");

        // WHILE looping is the same as in C, but IT IS SLOWER than FOR looping
        // when we loop through an array. Why?
        // Because the COMPILER ADDS RUNTIME CODE to perform the CONDITIONAL 
        // CHECK of whether the INDEX IS WITHIN THE BOUNDS of the array on every iteration through the loop

        // for loop with ranges
        println!("-- FOR LOOP WITH RANGES --");

        for nbr in (1..=5).rev()
        {
            print!("{nbr},");
        }
        println!();
    }

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