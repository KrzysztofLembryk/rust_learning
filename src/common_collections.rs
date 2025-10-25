
pub fn chapter_8_strings()
{
    // to string method
    let s = "Ala ma kota";

    // we want a String out of s
    let mut s_string = s.to_string();


    println!("################ Appending to a string #################");
    // Appending string to a string
    s_string.push_str(" thing to append ");

    println!("string after pushing string to it: {s_string}");
    // appending character to a string
    s_string.push('X');
    println!("string after pushing char to it: {s_string}");


    println!("################ String concatenation #################");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // we need a reference to &s2 since '+' in real is an add function that 
    // has signature: 
    //
    // fn add(self, s: &str) -> String { 
    //
    // so s1 ownership is taken by s3, while s2 will still exist 
    let s3 = s1 + &s2;

    println!("s2: {s2}, s3: {s3}");

    // The type of &s2 is &String, not &str, as specified in the second parameter to add. So why does it compile?

    // Because compiler is smart. Compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]

    // For concatening multiple string it's better to use format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("format macro output: {s}");


    println!("###########################################");
    println!("indexing strings - RUST DOES NOT SUPPORT IT");
    // indexing strings
    // RUST DOES NOT SUPPORT IT
    let s1 = String::from("ąhi");
    // let h = s1[0]; // gives error
    
    // since in string we can store i.e.: 'a' but also 'ą' and while 
    // 'a' takes one byte, 'ą' takes two bytes, thus s1[0] wouldn't be a 
    // valid character

    // So it's better to just use slices in this case
    let sliced_s = &s1[..2]; // &s1[..1] would panic since ą takes 2 bytes
    println!("sliced s: {sliced_s}");


    // #####################
    // ITERATING over String
    // #####################

    // if we want each character
    for c in s1.chars()
    {
        println!("{c}");
    }

    // we can also iterate over bytes using .bytes() method


    println!("#####################################");
    println!("############# CHANGING STRING ###############");
    // #################################
    // HOW TO change stuff inside String
    // #################################

    // we will again use chars() method to get an iterator over chars
    // then we will use map() and lambda expr that for each character will 
    // change it based on our rules
    let s: String = "Hello, world!"
                    .chars()
                    .map(|x| match x {
                        '!' => '#',
                        ',' => '%',
                        'A'..='Z' => 'X',
                        _ => x
                    })
                    .collect(); // transforms iterator into collection
    println!("String before : Hello, world!");
    println!("String changed: {s}");
}

pub fn chapter_8_vectors()
{

    // creating a new vector - we need to specify the type since we create an
    // empty vector (on heap), thus compiler doesn't know the type 
    let v1: Vec<i32> = Vec::new();

    // we can use vec! macro for creating a new vector
    let v2 = vec![1, 2, 69];

    // like any variable vec can be mut or immutable
    // we don't need type annotation since after creating vec3 we do the push()
    // operation, and compiler sees what we push and knows the vec type
    let mut vec3 = Vec::new();

    vec3.push(5);
    vec3.push(33);

    // Reading vector elements
    let v = vec!["a", "b", "cde", "xdxd"];

    // since this is a vec of str literals, by doing v[2] we get reference &str
    // it has a copy trait so both vector and third_val still have access to cde
    let third_val = v[2];
    println!("third val of vec: {third_val}");
    println!("vector: {v:?}");

    // vector of Strings
    let v_strings = vec![String::from("a"),
                        String::from("asd"), 
                        String::from("xdxd")];

    // if we did below we would get compile error, since we attempt to move String out of vector which is prohibited, String does not have Copy trait
    // let second_val = v_strings[1];

    // this is fine since we get the reference only, and not ownership
    let second_val = &v_strings[1];

    // best way of getting values from vector is: get()
    // we get an option of &String
    let second_val: Option<&String> = v_strings.get(22);

    match second_val
    {
        Some(val) => println!("second val using get is: {val}"),
        None => println!("INDEX OUT OF SOCPE using get")
    }
    let second_val: Option<&String> = v_strings.get(1);

    match second_val
    {
        Some(val) => println!("second val using get is: {val}"),
        None => println!("INDEX OUT OF SOCPE using get")
    }


    // we can also do this using let else
    let Some(val) = second_val else {
        println!("PANIC, second_val is None");
        return;
    };

    println!("val of let else expr: {val}");


    // immutable for looping over vector
    let mut v = vec![100, 32, 57];

    println!("iterating over vector: ");

    // we are iterating over references, so that ownership is not transferred
    for elem in &v 
    {
        print!("elem: {elem}, ");
    }
    println!();

    // mutable forr looping

    for elem in &mut v
    {
        // elem is a reference to the value stored in v
        // this if we want to change value the reference points to, like with
        // pointers in C, we need to dereference it using '*'
        *elem = 69;
    }
    println!("vector after we changed it in loop: {v:?}");

    // Dropping a Vector Drops Its Elements
}