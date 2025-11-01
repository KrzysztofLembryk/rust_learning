use std::cell::RefMut;
use std::cmp::Ordering;
use std::io;
use rand::Rng;

mod structs_enums;
mod common_collections;
mod error_handling;
mod generics;
mod minigrep;
mod iterators_closures;
mod smart_pointers;

use smart_pointers::{
    chapter_15_deref
};

use structs_enums::{chapter_5_1_structs, 
    chapter_5_2_exmpl_prog_with_structs,
    chapter_5_3_struct_methods,
    chapter_6_2_MATCH};

use common_collections::{
    chapter_8_vectors,
    chapter_8_strings
};


use error_handling::{
    chapter_9_recoverable_errors
};

use generics::{
    chapter_10_generics
};

use minigrep::{
    grep_main
};

use crate::smart_pointers::chapter_15_deref;

fn main() {
    // chapter_5_1_structs();
    // chapter_5_2_exmpl_prog_with_structs();
    // chapter_5_3_struct_methods();
    // chapter_6_2_MATCH();
    // chapter_8_vectors();
    // chapter_8_strings();
    // chapter_9_recoverable_errors();
    // chapter_10_generics();
    grep_main();
}


fn chapter_4_3_SLICES()
{
    let is_slices = true;
    let is_string_literals = true;

    if is_slices {
        println!("############ SLICES #############");
        // SLICES let you REFERENCE A CONTIGUOUS SEQUENCE OF ELEMENTS in a 
        // collection. A slice is a kind of reference, so it does not have ownership.

        // Programming Problem: write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned.

        // How to do it in rust, but without slices?

        println!("############ WITHOUT SLICES #############");
        // We don’t need the ownership of string, so we use reference.
        // Our function will return idx of the end of the word
        fn first_word(s: &String) -> usize 
        {
            // we want to go through string elem by elem and check if value is 
            // a space --> we convert our string to an ARRAY OF BYTES
            let bytes = s.as_bytes();

            // We create iterator over the bytes array using iter.() 
            // then we use enumerate() so that we have elements idxs
            // enumerate returns tuple --> (idx, &ref_to_data) 
            for (i, &item) in bytes.iter().enumerate()
            {
                if item == b' '
                {
                    return i;
                }
            }
            return s.len();
        }

        // We now have a way to find out the index of the end of the first word in the string, but there’s a problem. We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String. In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.

        // For example: 
        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        println!("initial string: {s}, idx of end of first word: {word}");

        s.clear(); // this empties the String, making it equal to ""

        println!("initial string: '{s}' (empty), idx of end of first word: {word} (the same as it was, but s is empty now)");
        // word still has the value 5 here, but s no longer has any content 
        // that we could meaningfully use with the value 5, so word is now totally invalid!

        // !!!!!!!!!!!!!1
        // Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
        // !!!!!!!!!!!!!1


        println!("############ WITH SLICES #############");

        // A string slice is a reference to a contiguous sequence of the elements of a String:
        // Internally, the SLICE DATA STRUCTURE STORES THE STARTING POSITION and the LENGTH OF THE SLICE (&s[..3] is a FAT pointer)
        // ---> 'str' is an unsized type (or dynamically sized type) 
        //      WHY? Because we can have str= "Ala ma kota", 
        //      but also str= "Ala", two different sizes (size not known at 
        //      compile time)

        let s = String::from("hello!world");
        println!("s = {s}");

        // #####################################################################
        // ############### EXPLANATION WHY WE USE & IN SLICES ##################
        // #####################################################################
        // s[..5] returns the ACTUAL DATA OF TYPE 'str', that is stored
        //      somewhere, we cannot do ```let s2 = s[..5]```, since rust will 
        //      want to move and transfer ownership from s to s2, so one part 
        //      of whole string would have different owner than the other part. 
        // 
        // Moreover 'str' is unsized type, rust doesn't allow to have 
        //      unsized types as variables, thus we need to make reference from 
        //      this str by using '&'. Reference &str is sized compared to str.
        //      So even when we do let s = "Trol", this 's' variable is '&str' 
        //      and not 'str', because in this case str has size 5 bytes but 
        //      '&str' has always size 16 bytes
        // 
        // By making reference &s[..4] we finally obtain SLICE, this reference
        //      is called FAT POINTER, since it stores pointer to the first elem
        //      defined in slice and length of the slice
        // #####################################################################
        // #####################################################################
        let hello1 = &s[0..5]; // we take elements from 0 to 4, without 5
        let hello2 = &s[0..=5]; // we take elements from 0 to 5
        let hello3 = &s[..5]; // from begginning to 4 idx

        println!("s[0..5]: '{hello1}', s[0..=5]: '{hello2}', s[..5]: '{hello3}'");

        // if we want to take all elems till the end we can omit second number!!
        let world = &s[6..]; 

        // we can take whole string if we do not supply any numbers
        let whole_string = &s[..];

        println!("s[6..]: '{world}', s[..]: '{whole_string}");

        // So now we can rewrite the first_word function:

        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // !!!!!!!!!!!!!!!!!!!! DIFFERENCE BTW str and String !!!!!!!!!!!!!!!!!!
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // --> A String type is a container for a str that is stored on the heap.
        // --> str is the actual text itself
        // --> &str is reference to the actual text itself stored on heap

        // #####################################################################
        // ########################## WHEN TO USE WHICH ########################
        // #####################################################################
        // --> Use &str when you KNOW THERE IS AN OWNER OF THE STRING ALREADY, and they will hold still for you to borrow it as long as you need it. 
        // --> If THERE IS NO EXISTING OWNER, or if the owner has its own business that is incompatible with you borrowing it, then YOU NEED TO USE STRING.

        // So any function which CREATES A NEW STRING that DID NOT PREVIOUSLY EXIST MUST RETURN STRING rather than &str, because in order to continue existing past the function returning, the string needs to be owned by the return value. 
        // In general function that creates sth can't return a reference to this
        // thing since created variable will be destructed after the function
        // ends, and we will have dangling reference (Rust compile won't allow that)

        // full expl: https://users.rust-lang.org/t/understanding-when-to-use-string-vs-str/103746/2

        fn first_word_better(s: &String) -> &str 
        {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate()
            {
                if item == b' '
                {
                    return &s[..i];
                }
            }
            return &s[..];
        }

        let s = String::from("test string xd");
        let x = first_word_better(&s); // immutable borrow

        println!("first len: {}, content: {}", x.len(), x);

        // we cannot clear our string now, as we could in first version of 
        // first_word function, since our x is an immutable reference to s
        // Not to whole 's', but to part of it, but it is enough, since if 
        // we cleared 's' now, we would have dangling immutable reference.

        // s.clear(); // mutable reference of 's' needed since clear() wants 
        // to truncate the String, and since println uses 'x', our immutable ref
        // is still valid, thus we cannot have both mutable and immutable at the
        // same time

        // println!("AFTER CLEAR: first len: {}, content: {}", first.len(), first);

    }

    if is_string_literals {
        println!("############ STRING LITERALS #############");
        // string literals (str type):
        // ---> are stored inside the binary read-only data section
        // ---> hardcoded in compiled programme
        // ---> STATIC lifetimes; they live for entire duration of programme

        let s = "Hello, world!";
        // The type of s here is &str: it’s a slice pointing to that specific point of the binary. This is also why string literals are immutable; &str is an immutable reference.

        // below function signature will work for both String and str
        fn first_word_best(s: &str) -> &str {
            s
        }
        // Why? If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the String or a reference to the String. This flexibility takes advantage of DEREF COERCIONS.

        // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality
    }
}

fn chapter_4_2_REFERENCES()
{
    // A REFERENCE IS LIKE A POINTER 
    // ---> it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
    let is_reference_borrowing = true;

    if is_reference_borrowing {
        println!("################# REFERENCES ###############");
        // But what to do when we want function to do stuff with our variable, 
        // but not take ownership of it? For example, calculate length of 
        // string? Currently we need to do it like that:
        fn calc_len(s: String) -> (String, usize)
        {
            let len = s.len();

            // In main scope we still want to use our string value, so we need 
            // to return it since ownership was moved, but we also want length
            // thus we need to return tuple
            (s, len)
        }

        let s = String::from("Calc len of string");

        // to calc length of string and still be able to use this string we 
        // need to return tuple - TEDIOUS
        let (s, len) = calc_len(s);
        println!("TEDIOUS WAY:");
        println!("string s={s}, its length: {len}");

        println!("######### REFERENCES and BORROWING ###########");
        // A reference is an address we can follow to access the data stored at that address; THAT DATA IS OWNED BY SOME OTHER VARIABLE. 
        // Unlike a pointer, a reference is GUARANTEED TO POINT TO A VALID VALUE of a particular type for the life of that reference.

        // Instead of giving up our ownership, we pass to function a REFERENCE
        // using '&' symbol. We now don't need to use tuples!!
        fn calc_len_with_reference(s: &String) -> usize 
        {
            s.len()
        } // // Here, s goes out of scope. But because s does not have ownership of what it refers to, the String is not dropped.

        let s = String::from("string for reference");
        let len = calc_len_with_reference(&s);

        println!("main scope Still has ownership of s={s}, s.len={len}");

        println!("############ MUTABLE REFERENCES #############");

        // ###################################
        // REFERENCES ARE IMMUTABLE BY DEFAULT
        // ###################################

        // When we create a reference from our s, since s is NOT MUTABLE, 
        // reference will also be IMMUTABLE
        // Thus below function will give us error, we cannot modify data from
        // immutable reference
        // fn change(some_string: &String) 
        // {
        //     some_string.push_str(", world");
        // }

        fn change_mut_red(ref_mutable_str: &mut String)
        {
            ref_mutable_str.push_str("string added in function");
        }

        // function takes a reference to mutable string, so for it to work we 
        // also need to create a mutable string
        let mut str_mut: String = String::from("mutable string ");

        // we give mutable reference to mutable string
        change_mut_red(&mut str_mut);

        println!("string modified in function using reference: '{str_mut}' ");

        // !!!!!!!!!!!!!!!!!!!!!!!
        // IF YOU HAVE A MUTABLE REFERENCE TO A VALUE, YOU CAN HAVE NO OTHER 
        // REFERENCES TO THAT VALUE
        // !!!!!!!!!!!!!!!!!!!!!!!

        // So we can have only one reference to a value when its mutable reference, but multiple references but only when they are IMMUTABLE 

        let mut s = String::from("some str");
        // this is fine since both references are immutable
        let ref_1 = &s;
        let ref_2 = &s;

        println!("ref1: {ref_1}, ref2: {ref_2}");

        // this will not work, we cannot have two mutable references to variable
        // at the same time
        // let mut_ref_1 = &mut s;
        // let mut_ref_2 = &mut s;

        // println!("mut ref1: {mut_ref_1}, mut ref2: {mut_ref_2}");

        // We CANNOT HAVE IMMUTABLE AND ONE MUTABLE REFERENCE at the same time, 
        // Users of an immutable reference don’t expect the value to suddenly change out from under them! However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data.

        // let ref_1 = &s;
        // let ref_2 = &s;
        // let mut_ref_3 = &mut s;

        // println!("ref1: {ref_1}, ref2: {ref_2}, mut ref {mut_ref_3}");


        // REFERENCE’S SCOPE STARTS FROM WHERE IT IS INTRODUCED AND CONTINUES THROUGH THE LAST TIME THAT REFERENCE IS USED

        // so below code will compile:
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("immutable refs: {r1} and {r2}");
        // Variables r1 and r2 will not be used after this point.

        let r3 = &mut s; // no problem
        println!("mutable ref: {r3}, it works, cause r1 and r2 immut refs are out of scope and no longer used when we use mutable r3");


        println!("########## DANGLING REFERENCES ##########");

        // In languages with pointers, it’s easy to erroneously create a 
        // dangling pointer—a pointer that references a location in memory that 
        // may have been given to someone else—by freeing some memory while 
        // preserving a pointer to that memory

        // In Rust, the COMPILER GUARANTEES THAT REFERENCES WILL NEVER BE 
        // DANGLING REFERENCES. 

        // below function will not compile and give below error:
        // this function's return type contains a borrowed value, but there is 
        // no value for it to be borrowed from

        // fn dangle() -> &String // dangle returns a reference to a String
        // { 
        //     let s = String::from("hello"); // s is a new String

        //     &s // we return a reference to the String, s
        // } // Here, s goes out of scope and is dropped, so its memory goes away.

        // Because s is created inside dangle, when the code of dangle is 
        // finished, s will be deallocated. But we tried to return a reference 
        // to it. That means this reference would be pointing to an invalid 
        // String. That’s no good! Rust won’t let us do this.


        // ########### RECAP ###############
        // At any given time, you can have either: 
        // ---> one mutable reference 
        // ---> or any number of immutable references.
        // References must always be valid.
    }
    
}

fn chapter_4_1_OWNERSHIP()
{
    let is_ownership = true;
    let is_string_type = true;
    let is_functions = true;

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

        // ############## CHANGING OWNERSHIP ETC ##############
        println!("########### Variables and Data Interacting with Move ###########");

        // since integers are simple data types with a known FIXED size at 
        // compile time, thus they will be copied and pushed on stack with '='
        // in their case SHALLOW COPY = DEEP COPY, so there is no need for clone
        // method
        let x = 5;
        let y = x;
        println!("x = {x}, y = {y}");

        // HOWEVER this is not the case with below example, since string is more
        // complex and have data allocated on heap
        let s1 = String::from("some string");
        let s2 = s1;

        // below will give an error: 
        // println!("s1 is invalid {s1}");

        // s2 will not have a copy of s1, but s2 will BE THE NEW AND ONLY OWNER
        // of s1 string data, thus after s2 = s1 line, s1 is NO LONGER VALID
        // and does not point to our String data. Why? Because if it did point, 
        // after going out of scope rust would do free(s2) AND ALSO free(s1) and
        // we would have double free bug.

        // Thus in ```let s2 = s1;``` rust performs MOVE operation, 
        // so s1 is not valid anymore, and s2 has its data (s2 copies pointer 
        // to string data that is allocated on heap, so we have shallow copy)

        // ############## DEEP COPY / SHALLOW COPY ##############

        // Also RUST WILL NEVER AUTOMATICALLY CREATE “DEEP” COPIES OF YOUR DATA 
        // Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance

        // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately

        let mut mut_str = String::from("This val will be dropped");
        mut_str = String::from("This val will appear");

        println!("previous mut_str val was dropped: {mut_str}");

        // When we want deep copy we use clone method

        let s1 = String::from("this string will be copied");
        let s2 = s1.clone();

        println!("######### DEEP COPY using clone() ################"); 
        println!("s1 = {s1}, s2 = {s2}");


        // Rust has a special annotation called the COPY TRAIT that we can place on types that are stored on the stack. If a type implements the Copy trait, VARIABLES THAT USE IT DO NOT MOVE, BUT ARE TRIVIALLY COPIED making them still valid after assignment to another variable.
    }

    if is_functions {
        println!("############## OWNERSHIP WITH FUNCTIONS ##############");

        fn take_ownership(s: String)
        {
            println!("I have taken ownership of s='{s}', s is no longer valid outside of this function :)");
        }

        fn makes_copy(x: i32)
        {
            println!("My arg is integer, so Deep copy = Shallow copy, so integers have COPY TRAIT, thus I got a copy in my argument, x={x}");
        }

        fn give_ownership() -> String
        {
            // we create new string, and then return it, thus this new_s loses
            // its ownership of string, but variable that will get value from
            // this function will get also an ownership of this string
            let new_s = String::from("new string");
            new_s
        }

        { // scope
            // mechanics of passing a value to a function are similar to those 
            // when assigning a value to a variable. 
            // Passing a variable to a function will move or copy
            let s = String::from("string to be moved to function");

            // here s is moved since it is complex type, doesnt have COPY TRAIT
            take_ownership(s);

            // therefore after take_ownership() function, s is no longer valid
            // in this scope, cause it's value was moved inside the function

            // below code will not compile
            // println!("s is not valid: {s}");

            // however with types that have copy trait values will be just copied, and variable in this scope after function will be valid
            let x = 32;
            makes_copy(x);

            println!("x={x} still valid after makes_copy function");

            // The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
            let s: String = give_ownership();
            println!("var s={s} in scope was given ownership by function");

            // While this works, taking ownership and then returning ownership with every function is a bit TEDIOUS. What if we want to let a function use a value but not take ownership
        }

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
