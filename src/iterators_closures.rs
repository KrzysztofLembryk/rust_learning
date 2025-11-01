use std::thread;

pub fn chapter_13_iterators()
{
    // The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. 

    // In Rust, ITERATORS ARE LAZY, meaning they have no effect until you call methods that consume the iterator to use it up

    // Example
    let v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter(); // creating iterator for the collection

    // now we can iterate over the vector:
    for val in v1_iter
    {
        println!("got: {val}");
    }

    // When we write below, this does what we did above but implicitly
    for val in v1
    {
        println!("implicit iter created, got: {val}");
    }

    // All iterators implement a trait named Iterator
    pub trait Iterator {
        type Item; // associated type with this trait

        fn next(&mut self) -> Option<Self::Item>;

    }
    // What is type Item?
    // implementing the Iterator trait requires that you also define an Item type, and this Item type is used in the RETURN TYPE OF THE NEXT METHOD. In other words, the Item type will be the type returned from the iterator.

    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // we needed to make v1_iter MUTABLE: calling the next method on an iterator CHANGES INTERNAL STATE iterator uses to keep track of where it is in the sequence. In other words, this code consumes the iterator

    // next() method consumes iterator

    // --> The iter method produces an iterator over immutable references
    // --> iter_mut - mutable references
    // --> into_iter - iterator that takes ownership

    // 
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum(); // iterator is consumed by using sum
    
    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on

    assert_eq!(total, 6);

    // ITERATOR ADAPTERS - methods that produce other iterators

    let v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1) - creates a new iterator that increases ecery element value by 1. However, ITERATORS ARE LAZY, thus we need to use collect
    // to see the effects
    // 
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // CLOSURE CAPTURING VARIABLES IN ITERATOR
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    };

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>
    {
        // -- We use into_iter, because we want to get ownership of shoes that
        // have right size and are in vector, 
        // -- Shoe struct does not know how to copy itself, 
        // -- Our function also returns Vector of Shoe, not of references
        // to Shoe
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    // Good example of how to use methods, iterators and just functional style
    // of programming
    fn bad_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }

        results
    }


    fn GOOOD_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> 
    {
        contents     
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }


}

pub fn chapter_13_closures()
{
    // CLOSURES
    // Closures are anonymous functions you can save in a variable or pass as arguments to other functions

    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue
    }

    struct Inventory
    {
        shirts: Vec<ShirtColor>
    }

    impl Inventory
    {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor
        {
            // 
            // The unwrap_or_else method on Option<T> is defined by the standard library. It takes one argument: a CLOSURE WITHOUT ANY ARGUMENTS that RETURNS A VALUE T (the same type stored in the Some variant of the Option<T>, in this case ShirtColor)
            //
            // The closure CAPTURES AN IMMUTABLE REFERENCE to the self
            user_preference.unwrap_or_else(|| self.most_stocked())

            // MORE:
            // --> Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do
            //
            // --> Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the COMPILER CAN INFER THE TYPES OF THE PARAMETERS AND THE RETURN TYPE, similar to how it’s able to infer the types of most variables
        }

        fn most_stocked(&self) -> ShirtColor
        {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts 
            {
                match color 
                {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue 
            {
                ShirtColor::Red
            } 
            else 
            {
                ShirtColor::Blue
            }
        }
    }

    { // exmpl
        let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    // We can add type annotations to closures
    let annotated_closure = |x: i32| x + 1;
    println!("annotated_closure res: {}", annotated_closure(3));

    // Different ways of creating closure syntax:
    // - fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    // - let add_one_v2 = |x: u32| -> u32 { x + 1 }; // fully annotated closure
    // - let add_one_v3 = |x|             { x + 1 };
    // - let add_one_v4 = |x|               x + 1  ;

    // For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value. 
    // 
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    // let n = example_closure(5); // ERROR
    // Why? The first time we call example_closure with the String value, the compiler infers the type of x and the return type of the closure to be String. Those types are then locked into the closure in example_closure, and we get a type error when we next try to use a different type with the same closure.


    // ###############################################################
    // ########### OWNERSHIP && REFERENCES WITH CLOSURES #############
    // ###############################################################

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // here we only print list, so closure gets IMMUTABLE REFERENCE
    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    // we have mutable list
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Here we push to the list, so compiler knows that we need mutable ref
    let mut borrows_mutably = || list.push(7);

    // this will result in error since immutable borrow to print isn’t allowed because NO OTHER BORROWS ARE ALLOWED WHEN THERE’S A MUTABLE BORROW
    // println!("Before calling closure: {list:?}"); 

    borrows_mutably();
    println!("After calling closure: {list:?}");

    // If we want to FORCE closure to take ownership, even though we do not
    // modify anything, we can with 'MOVE' keyword

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("from thread, forced move: {list:?}"))
        .join()
        .unwrap();

    // IF THE MAIN THREAD MAINTAINED OWNERSHIP OF LIST BUT ENDED BEFORE THE NEW THREAD and drops list, the immutable REFERENCE IN THE THREAD WOULD BE INVALID


    // ###############################################################
    // ######################## FN TRAITS ############################
    // ###############################################################

    // A closure body can do any of the following: 
    // --> move a captured value out of the closure, 
    // --> mutate the captured value, 
    // --> neither move nor mutate the value, 
    // --> capture nothing from the environment to begin with.


    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values

    // 
    // --> **FnOnce** applies to closures that CAN BE CALLED ONCE. A closure that MOVES CAPTURED VALUES OUT OF ITS BODY will only implement FnOnce and none of the other Fn traits because it can only be called once.
    // 
    // --> **FnMut** applies to closures that DON’T MOVE CAPTURED VALUES OUT OF THEIR BODY, but that MIGHT MODIFY the captured values. These closures can be called more than once.
    // 
    // --> **Fn** DON’T MOVE CAPTURED VALUES out of their body and that DON’T MODIFY CAPTURED VALUES, as well as closures that CAPTURE NOTHING. Can be called more than once. 

    // Example:
    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f: F) -> T
    //     where
    //         F: FnOnce() -> T // F is any type that implements FnOnce trait
    //                          Using FnOnce in the trait bound expresses the   
    //                          constraint that unwrap_or_else is only going to 
    //                          call f at most one time
    //     {
    //         match self {
    //             Some(x) => x,
    //             None => f(), // only place we use 'f' is here
    //         }
    //     }
    // }
    // 
    // Because all closures implement FnOnce (cause every closure can be called), unwrap_or_else accepts all three kinds of closures and is as flexible as it can be.

    // FnMut example:
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    // The reason sort_by_key is defined to take an FnMut closure is that it CALLS CLOSURE MULTIPLE TIMES: once for each item in the slice. 
    // The closure |r| r.width doesn’t capture, mutate, or move anything out from its environment, so it meets the trait bound requirements.
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");

    // Results in ERROR 
    // The closure CAPTURES VALUE and THEN MOVES VALUE OUT OF THE CLOSURE by transferring ownership of value to the sort_operations vector.
    // This closure can be called once; trying to call it a second time wouldn’t work because value would no longer be in the environment
    // 
    // 
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    println!("{list:#?}");

}