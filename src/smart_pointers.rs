use std::ops::Deref;

pub fn chapter_15_deref()
{
    // Deref trait allows you to customize the behavior of the dereference operator *
    // 
    // By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.
    // 

    // ########################################################################
    // ##################### HOW NORMAL REFERENCE WORK ########################
    // ########################################################################

    let x = 5;
    let y = &x;

    // The variable x holds an i32 value 5. 
    // We set y equal to a reference to x.  
    // If we want to make an assertion about the value in y, we have to use *y to follow the reference to the value it’s pointing to so the compiler can compare the actual value.
    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    // Thanks to Box<T> implementing Deref trait we can also do the following:
    let z = Box::new(x);

    // However this time in z we made a copy of x
    assert_eq!(*z, 5);

    // 
    // ########################################################################
    // ##################### IMPLEMENTING SMART POINTER #######################
    // ########################################################################
    // Box<T> type is defined as tuple struct with one elem

    // Syntax of tuple struct: struct Name<T, U> (T, U, T, ....)
    struct MyBox<T>(T);

    impl<T> MyBox<T>
    {
        fn new(x: T) -> MyBox<T>
        {
            MyBox(x)
        }
    }

    // Now we need to implement deref trait
    impl<T> Deref for MyBox<T>
    {
        type Target = T; // associated type, what deref will return

        fn deref(&self) -> &Self::Target 
        {
            // we return reference to the value stored in MyBox, so that we can
            // use deref '*' operator
            &self.0 
        }
    }

    let my_box = MyBox::new(x);

    // works like before
    assert_eq!(*my_box, 5);

    // when we did *my_box, in reality Rust did the following:
    // *(my_box.deref())

    // #####################################################################
    // ######## Implicit Deref Coercions with Functions and Methods ########
    // #####################################################################
    // 
    // Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type
    // 
    // For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str
    // 
    // It happens automatically, otherwise we woul need to add many explicit references and dereferences with & and *

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    // Exmpl with deref:
    let m = MyBox::new(String::from("some string"));
    hello(&m); // automatically &MyBox<String> converted to &str

    // Without automatic deref coearcion we would need to do:
    hello(&(*m)[..]);
    // *m - we get the String
    // &(*m) - we take ref of String (&String)
    // &(*m)[..] - we take slice of whole string (&str)
    

}

fn chapter_15_Box_pointer()
{
    // SMART POINTERS are data structures that act like a pointer, but also have additional metadata and capabilities
    // 
    // In Rust, while references only borrow data, in many cases SMART POINTERS OWN THE DATA THEY POINT TO
    // 
    // Smart pointers implement the DEREF and DROP traits:
    // 
    // --> Deref trait allows an instance of the smart pointer struct to BEHAVE LIKE A REFERENCE so you can write your code to work with either references or smart pointers (so i.e. dereferencing works with just '*' sign, without any other stuff)
    // 
    // --> Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope

    // ########################################################################
    // ######################## BOX SMART POINTER #############################
    // ########################################################################
    // 
    // It's somewhat similar to Unique pointer in C++
    // 
    // Boxes allow you to store data on the heap rather than the stack.
    // What REMAINS ON THE STACK IS THE POINTER TO THE HEAP DATA.
    // Use cases:
    // 
    // -- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // 
    // -- When you have a large amount of data and you want to TRANSFER OWNERSHIP but ENSURE THE DATA WON’T BE COPIED when you do so. When we use BOX only pointer to the data is being copied, and data stays on heap.
    // 
    // -- When you want to own a value and you care only that it’s a TYPE THAT IMPLEMENTS A PARTICULAR TRAIT rather than being of a specific type
    //

    // Syntax:
    let b = Box::new(69);
    println!("value in a box: {b}");

    // ########################################################################
    // #################### RECURSIVE TYPES WITH BOXES ########################
    // ########################################################################
    // 
    // A value of a recursive type can have another value of the same type as part of itself. Recursive are an issue since Rust needs to know at compile time how much space a type takes up. The nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs.
    // Boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition

    // ###### We will implement cons list (like in haskell) ######
    // 
    // Below will not compile since Rust cannot establish how much memory it 
    // needs, because of the recursion
    // 
    // enum List 
    // {
    //     Cons(i32, List), // Recursion
    //     Nil
    // }
    // 
    // Exmpl: let a = Cons(1, Cons(2, Cons(3, Nil)));

    // ############ Computing size of Non-recursive type ####################
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    // Rust sees that Message::Quit doesn’t need any space, Message::Move needs enough space to store two i32 values, and so forth. Because only one variant will be used, the most space a Message value will need is the space it would take to STORE THE LARGEST OF ITS VARIANTS.

    // ######################## Box type to rescue ###########################
    // Box<T> is a pointer, so it's size is always the same and known. 
    // Pointer size doesn't change based on data it points
    // This means we can put a Box<T> inside the Cons variant instead of another List value directly. The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant

    enum List {
        Cons(i32, Box<List>), // tuple of int value and pointer to another List
        Nil
    }
    //
    // The Cons variant needs the size of an i32 plus the space to store the box’s pointer data
    // We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data. By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.
    use List::{Cons, Nil};

    let lst = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}