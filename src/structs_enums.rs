
// To define a struct, we enter the keyword struct and name the entire struct.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

pub fn chapter_6_3_if_let()
{
    // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest

    #[derive(Debug)] 
    enum UsState {
        Alabama,
        Alaska,
    }

    impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), 
        // enums can have values and in match expr we can get them
    }

    // so instead of writing below:
    let config_max = Some(8);

    match config_max 
    {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }

    // We can write it using if let.
    // Basically if our config_max is of form Some(value), we will run the code
    // inside if, also we will have access to value stored inside Some
    if let Some(max) = config_max
    {
        println!("if let maximum: {max}");
    }

    // if let ... ELSE syntax means basically the same as our match expression
    // above

    if let Some(max) = config_max
    {
        println!("we are in if let, not in else, max: {max}");
    }
    else  // in match this is case: _ => ()
    {
        println!("else branch, default behaviour if config_max is not Some(val)");
    }

    // let ... else syntax
    fn describe_coin(coin: Coin) -> Option<String>
    {
        // if coin is of type Quarter we will execute rest of the function
        // but if it's not we will immediately return with None
        let Coin::Quarter(state) = coin else {return None;};

        // If coin is Quarter, by doing let ... else, we obtained 'state' value 
        // from inside of the Coin, and can use this state variable in our function
        if state.existed_in(1900)
        {
            Some(format!("{state:?} is pretty old, for America!"))
        }
        else 
        {
            Some(format!("{state:?} is relatively new."))     
        }
    }

}

pub fn chapter_6_2_MATCH()
{
    // Values go through each pattern in a match, and at the first pattern the value “fits,” the value falls into the associated code block to be used during execution

    #[derive(Debug)] 
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState), 
        // enums can have values and in match expr we can get them
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                // here we are getting inner state of Quarter
                println!("state is: {state:?}, value is: 25");
                25
            }
        }
    }

    let one_quarter = Coin::Quarter(UsState::Alabama);
    value_in_cents(one_quarter);

    // we do the same with Option
    fn plus_one(x: Option<i32>) -> Option<i32> {
        // !!! the arms’ patterns must cover all possibilities
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // OTHER keyword - useful when we want to handle only a few cases and for the rest there is one way of handling

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // last pattern will match all values not specifically listed. This catch-all pattern meets the requirement that match must be exhaustive

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    } // we want to catch but dont care about value we use '_' (like in haskell)

    // if we dont want to do anything in given arm, we can do below:
    // _ => ()

    fn reroll() {}
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

pub fn chapter_6_2_OPTION_enum()
{
    // Option type encodes the very common scenario in which a value could be something or it could be nothing

    // !!! Rust DOESN’T HAVE the NULL FEATURE that many other languages have

    // The problem with null values is that if you try to use a null value as a not-null value, you’ll get an error of some kind. 

    // However, the concept that null is trying to express is still a useful one: a null is a value that is currently invalid or absent for some reason.

    // Thus instead of NULL Rust has OPTION enum
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // Its variants are also included in the PRELUDE: you can use Some and None directly without the Option:: prefix

    // here Rust will infer types
    let some_number = Some(5);
    let some_char = Some('e');

    // but when assigning None we have to specify the type, because
    // compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None
    let absent_number: Option<i32> = None;

    // ######################## WHY OPTION IS GOOD #############################
    // You have to convert an Option<T> to a T before you can perform T 
    // operations with it. Generally, this helps catch one of the most common 
    // issues with null: assuming that something isn’t null when it actually is.
    // ########################################################################

    // Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null
}

pub fn chapter_6_1_enums()
{

    // enums basics
    {
        // Any IP address can be either a version four or a version six address, but not both at the same time. That property of IP addresses makes the enum data structure appropriate because an enum value can only be one of its variants
        enum IpAddrKind 
        {
            V4,
            V6
        }
    
        // creating an instance
        let ip_four = IpAddrKind::V4;
        let ip_six = IpAddrKind::V6;
    
        // both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind
        // so we can make  a function that takes thi IpAddrKind
        // fn route(ip_kind: IpAddrKind) {} and it will work for both V4 and V6
    }

    // Enums with data
    {
        // !!! We can also put DATA DIRECTLY INTO ENUM, so instead of making a struct:
        // struct IpAddr {type: IpAddrKind, addr: String}
        // !!! we can do:
        enum IpAddr 
        {
            V4(String),
            V6(String)
        }

        // And now we can store data in enum
        let my_ip = IpAddr::V4(String::from("127.0.0.1"));

        // The name of each enum variant that we define also becomes a function that constructs an instance of the enum. That is, IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type
    }

    // Enums with different types of data
    {
        // Each variant in enum CAN HAVE A DIFFERENT TYPE
        enum IpAddr 
        {
            V4(u8, u8, u8, u8), // we can put structs here also
            V6(String)
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    // More complex enum
    {
        enum Message {
            Quit, // no data associated with Quit
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        // We can define methods on enums
        impl Message 
        {
            fn call(&self)
            {
                unimplemented!()
            }
        }
    }
}

pub fn chapter_5_3_struct_methods()
{

    #[derive(Debug)]
    struct Rectangle {
        width: u32, // these are currently public
        height: u32,
    }

    // Struct Simple methods
    {
        println!("################ STRUCT SIMPLE METHODS ################");

        // Struct methods:
        // their first parameter is always SELF, which represents the instance of the struct the method is being called on (like in python).


        // To implement methods for given struct we use 'impl' word
        // Everything within this 'impl' block will be associated with the Rectangle type
        impl Rectangle 
        {
            // The &self is actually short for self: &Self. Within an impl block, 
            // the type Self is an alias for the type that the impl block is for. 
            // --> Methods MUST HAVE A PARAMETER NAMED SELF of type Self FOR THEIR FIRST PARAMETER
            fn area(&self) -> u32 
            {
                // We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it
                self.width * self.height
            }

            // Having a method that TAKES OWNERSHIP BY USING JUST SELF is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

            // We can define getters; Rust does not implement them automatically for struct fields as some other languages do
            fn width(&self) -> u32 
            {
                self.width
            }

        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.(calculated by area() method)",
            rect1.area()
        );
    }

    // !!!!!!!!!!!!!111
    // WHERE IS -> OPERATOR? (like in c++)
    // !!!!!!!!!!!!!!!!
    {
        // In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

        // Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called AUTOMATIC REFERENCING AND DEREFERENCING used when calling methods.

        // When you call a method with 'object.something()', Rust AUTOMATICALLY ADDS in '&,' '&mut', or '*' so OBJECT MATCHES THE SIGNATURE OF THE METHOD

        // for example:
        // p1.distance(&p2) === (&p1).distance(&p2)

        // This automatic referencing behavior works because METHODS HAVE A CLEAR RECEIVER—THE TYPE OF SELF
        // Rust can figure out definitively whether the method is reading      (&self), mutating (&mut self), or consuming (self)

        // exmpl:
        impl Rectangle 
        {
            fn diff(&self, rect: &Rectangle) -> u32
            {
                self.width * self.height - rect.width * rect.height
            }
        }


        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangle {
            width: 10,
            height: 20,
        };

        println!("diff of surfaces: {}", (&rect1).diff(&rect2)); 
        // the same as rect1.diff(&rect2)
        // we wrote: (&rect1).diff(&rect2), since in diff signature we 
        // have &self, so our &rect1 is this &self
    }

    // Associated functions
    {
        // All functions defined within an impl block are called associated functions

        // We can DEFINE ASSOCIATED FUNCTIONS THAT DON’T HAVE SELF as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with

        impl Rectangle
        {
            // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle
            fn square(x: u32) -> Self
            {
                Self { width: x, height: x }
            }
        }

        let sq1 = Rectangle::square(69);

        dbg!(&sq1);

    }

    // !!!! Each struct is allowed to have multiple impl blocks
}

pub fn chapter_5_2_exmpl_prog_with_structs()
{
    // calc area without structs
    {
        println!("############## AREA WITHOUT ANYTHING ################");
        // ISSUE: The 'area' function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it’s not clear anywhere in our program that the parameters are related

        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );

        fn area(width: u32, height: u32) -> u32 {
            width * height
        } 

    }

    // Area with tuples
    {
        println!("############## AREA WITH TUPLES ################");
        // ISSUE:
        // We’re now passing just one argument, that's good. 
        // But in another way, this version is less clear: tuples don’t name their elements, so we have to index into the parts of the tuple, making our calculation less obvious, we need to remember which index is width and which is height - bug prone
      

        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area(rect1)
        );

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

    }

    // Aread with structs - best way
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        fn area(rectangle: &Rectangle) -> u32 {
            // Accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs
            rectangle.width * rectangle.height
        }
    }

    // Derived traits, dbg! macro
    {
        // By default structs don't have Display trait implemented, so we cannot
        // do the following:
        // println!("react = {rect1}");

        // We can try with :?, but still it won't work
        // println!("rect1 is {rect1:?}");

        // for :? to work we need to add debug macro explicitely to our struct

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!("rect1 with derive(Debug) macro added is {rect1:?}");

        println!("Using dbg! macro:");
        // !!!!!
        // dbg! macro, which TAKES OWNERSHIP of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and RETURNS OWNERSHIP of the value.
        // !!!!!

        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale),
            height: 50,
        };

        dbg!(&rect1); // we don't want dbg! macro to take ownership thus ref
    }
}

pub fn chapter_5_1_structs()
{
    // STRUCTS BASICS 
    {
        println!("##################### STRUCTS BASICS ####################");
        // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields. We do it by using key:value pairs; they do not need to be in the same order as in struct
        let user1 = User { // immutable user
            active: true,
            email: String::from("xdxd@xmail.comx"),
            username: String::from("first_user"),
            sign_in_count: 1
        };

        // To get a specific value from a struct, we use dot notation
        println!("User1 name: {}", user1.username);


        // !!!!
        // Entire instance MUST BE MUTABLE; 
        // Rust doesn’t allow us to mark only certain fields as mutable
        // !!!!
        let mut user2_mut = User { // mutable user
            active: true,
            email: String::from("mut@xmail.comx"),
            username: String::from("second_user_mut"),
            sign_in_count: 1
        };

        user2_mut.email = String::from("no email at all");

        println!("User2 mail: {}", user2_mut.email);

        // email and username will have their ownership transferred to the User
        fn bad_build_user(email: String, username: String) -> User 
        {
            User {
                email: email,
                username: username,
                active: true,
                sign_in_count: 1
            }
        }
        // It makes sense to name the function parameters with the same name as the struct fields, but having to repeat the email and username field names and variables is a bit tedious.

        let user_built = bad_build_user(String::from("malpa@fmail.com"), String::from("Tomasz"));
        println!("user built: {}, email:{}", user_built.username, user_built.email);
    }

    // FIELD INIT SHORTHAND
    {
        println!("################### Field Init Shorthand ##################");
        // Because the parameter names and the struct field names are exactly the same in Listing 5-4, we can use the field init shorthand syntax to rewrite build_user so it behaves exactly the same but doesn’t have the repetition of username and email

        fn good_build_user(email: String, username: String) -> User {
            User {
                active: true,
                username, // no repetition
                email, // no repetition
                sign_in_count: 1,
            }
        }
    }

    // INSTANCES FROM OTHER INSTANCES
    {
        println!("######### CREATING INSTANCES FROM OTHER INSTANCES #########");

        // It’s often useful to create a new instance of a struct that includes most of the values from another instance of the same type, but changes some
        let user1 = User { 
            active: true,
            email: String::from("xdxd@xmail.comx"),
            username: String::from("first_user"),
            sign_in_count: 1
        };

        let user2 = User {
            username: String::from("copied user"),
            ..user1
        };

        // The syntax '..' specifies that the remaining fields not explicitly 
        // set should have the same value as the fields in the given instance.
        // The ..user1 must come last

        // !!!!!!!!!!!!!!!!!!!
        // We used '=' operation in our Struct Update Syntax, so after 
        // creating user2, values from user1 WERE MOVED TO USER2.
        // ---> user1 no longer has email, cause its value was moved to user2
        // ---> however it still has ownership of username so:

        println!("user1 username: {}", user1.username); // this works

        // println!("user1 email: {}", user1.email); // this doesnt

        // !!!!!!!!!!!!!!!!!!!!
    }

    // TUPLE STRUCTS
    {
        println!("######### TUPLE STruCTS #########");
        
        // Tuple structs have the struct name but don’t have names associated with their fields; they just have the types of the fields
        // Each struct you define is its own type, even though the fields within the struct might have the same types
        struct Color (i32, i32, i32);
        struct Point (i32, i32, i32);
        struct FullName (String, String);
        
        let full_name = FullName(String::from("Gierward"), String::from("Kot"));
        let origin = Point(0, 0, 0);

        // we access values the sam as in tuples
        println!("origin ({}, {}, {})", origin.0, origin.1, origin.2);

        // The same as with tuples we can DESTRUCTURE them, but while doing so 
        // we need to supply struct name
        let Point(x, y, z) = origin;

        // let (x, y, z) = origin; // won't work

        // when destructing types that don't have copy trait we MOVE values
        let FullName(name, surname) = full_name;

        println!("name: {}, surname: {}", name, surname);

        // this will give error, since we moved theses values earlier
        // println!("name: {}, surname: {}", full_name.0, full_name.1);


        println!("##### Unit-Like Structs Without Any Fields #####");
        // Structs that don’t have any fields, these are called unit-like structs because they behave similarly to ()
        // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself

        struct AlwaysEqual;

        let subject = AlwaysEqual;

        // later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type
    }

}