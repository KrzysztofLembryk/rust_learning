use std::cmp;
use std::fmt::Display;
use std::fmt::Debug;

pub fn chapter_10_lifetimes()
{
    // Lifetimes are another kind of generic that we’ve already been using. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

    // Every reference in Rust has a lifetime, which is the scope for which that reference is valid. Most of the time, lifetimes are implicit and inferred


    // !!!!!!!!!!!1
    // The main aim of lifetimes is to prevent DANGLING REFERENCES

    {
        let r = 4;

        // This will not compile 
        {
            let x = 5;
            // r = &x; // commented so that code compiles
        } // here x goes out of scope

        // thus we have dangliing reference in r cause x does not exist anymore
        println!("r: {r}");


        // How does compiler know that this would be a dangling reference?
        // Because it has a BORROW CHECKER
        // This is how borrow checker sees code above
        //
        // We have:
        // --> r lifetime = 'a 
        // --> x lifetime = 'b 
        // let r;                  // ---------+-- 'a
        //                         //          |
        // {                       //          |
        //     let x = 5;          // -+-- 'b  |
        //     r = &x;             //  |       |
        // }                       // -+       |
        //                         //          |
        // println!("r: {r}");     //          |
        //                         // ---------+
        // inner 'b block is much smaller than the outer 'a lifetime block. At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
    }

    {
        let x = 5;              // ----------+-- 'b
                                //           |
        let r = &x;             // --+-- 'a  |
                                //   |       |
        println!("r: {r}");     //   |       |
                                // --+       |
    }                           // ----------+
    // Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.

    // Generic Lifetimes in Functions
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        // we want the function to take string slices, which are references, rather than strings, because we don’t want the longest function to take ownership of its parameters
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");

        // If we impl longest func like below, program WON'T COMPILE
        // 
        // fn longest(str1: &str, str2: &str) -> &str
        // {
        //     if str1.len() > str2.len() {str1} else {str2}
        // }
        // 
        // We get below error msg:
        // this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `str1` or `str2`

        // Return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y. Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y

        // When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute. We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did earlier to determine whether the reference we return will always be valid. 


        // ##########################
        // LIFETIME ANNOTATION SYNTAX
        // ##########################
        // 
        // Lifetime annotations DON’T CHANGE HOW LONG ANY OF THE REFERENCES LIVE. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes
        //
        //  We place lifetime parameter annotations after the &

        // To use lifetime annotations in function signatures, we need to below:
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str 
        {
            if x.len() > y.len() { x } else { y }
        }

        // !!!!!!!!!!!!!!!!!!
        // The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that LIVE AT LEAST AS LONG AS LIFETIME 'a. 
        // 
        // The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. 
        // 
        // IN PRACTICE, it means that the LIFETIME OF THE REFERENCE RETURNED is the SAME AS THE SMALLER OF THE LIFETIMES of the values referred to by the function arguments
        // !!!!!!!!!!!!!!!!!!

        // this will work cause result will have string2 lifetime and we use 
        // result within that lifetime
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {result}");
        }

        // this will not compile cause result = longest() will have smaller 
        // lifetime, meaning string2 lifetime, but we use it outside of the 
        // inner scope, thus we have dangling reference 
        // 
        // let string1 = String::from("long string is long");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(string1.as_str(), string2.as_str());
        // }
        // println!("The longest string is {result}");
    }

    // Lifetimes in STRUCTS
    {
        // We can define STRUCTS TO HOLD REFERENCES, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition

        // Lifetime annotation means an instance of ImportantExcerpt CAN’T OUTLIVE THE REFERENCE IT HOLDS in its part field.
        struct ImportantExcerpt<'a>
        {
            part: &'a str, // we hold string slice which is a reference
            other_val: usize,
        } // so importantExcerpt objects can live only as long as 'a (part var)

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
            other_val: 69
        };

        // LIFETIME ELLISIONS
        // The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations

        // 1) The first rule is that the COMPILER ASSIGNS A LIFETIME PARAMETER TO EACH PARAMETER THAT’S A REFERENCE. 
        // --> a function with one param gets one lifetime parameter: 
        //     fn foo<'a>(x: &'a i32); 
        // --> a function with two params gets two separate lifetime params: 
        //     fn foo<'a, 'b>(x: &'a i32, y: &'b i32) ...

        // 2) The second rule is that, IF THERE IS EXACTLY ONE INPUT LIFETIME PARAMETER, THAT LIFETIME IS ASSIGNED TO ALL OUTPUT LIFETIME PARAMETERS: 
        // --> fn foo<'a>(x: &'a i32) -> &'a i32. 

        // 3) The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the LIFETIME OF SELF IS ASSIGNED TO ALL OUTPUT LIFETIME PARAMETERS

        // Let's apply these rules to our longest function:
        // --> fn longest(x: &str, y: &str) -> &str 
        // 
        // Let’s apply the first rule: each parameter gets its own lifetime:
        // --> fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str 

        // You can see that the SECOND RULE DOESN’T APPLY because there is more than one input lifetime. The THIRD RULE DOESN’T APPLY either, because longest is a function rather than a method, so none of the parameters are self

        // THUS compiler doesn't know lifetime of returned value and that's why
        // we need to annotate it

        // #############################################
        // IMPLEMENTING METHODS FOR STRUCT WITH LIFETIME
        // #############################################

        // Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name because those lifetimes are part of the struct’s type.
        impl<'a> ImportantExcerpt<'a>
        {
            fn level(&self) -> i32 
            {
                3
            }

            // third lifetime elision rule applies
            // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
            fn announce_and_return_part(&self, announcement: &str) -> &str
            {
                println!("Attention please: {announcement}");
                self.part
            }
        }
    }

    // STATIC LIFETIME
    {
        // 'static, which denotes that the affected REFERENCE can LIVE FOR THE ENTIRE DURATION OF THE PROGRAM

        //  All string literals have the 'static lifetime, since The text of this string is stored directly in the program’s binary, which is always available
        let s: &'static str = "I have a static lifetime.";

        // You might see suggestions in error messages to use the 'static lifetime. But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to. Most of the time, an error message suggesting the 'static lifetime RESULTS FROM ATTEMPTING TO CREATE A DANGLING REFERENCE or a MISMATCH OF THE AVAILABLE LIFETIMES.
    } 

}

pub fn chapter_10_generics()
{
    // FUNCTIONS
    {
        // without generics
        fn largest_i32(list: &[i32]) -> &i32 
        {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        fn largest_char(list: &[char]) -> &char 
        {
            let mut largest = &list[0];

            for item in list {
                if item > largest {
                    largest = item;
                }
            }

            largest
        }

        // WITH GENERICS
        // we need to add to type T trait that T needs to know how to compare
        fn largest_generic<T: cmp::PartialOrd>(lst: &[T]) -> &T
        {
            let mut largest = &lst[0];

            for item in lst
            {
                if item > largest
                {
                    largest = item;
                }
            }

            largest
        }
    }

    // STRUCTS
    {
        struct Point<T, U>
        {
            x: T,
            y: U
        }

        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        let mixed = Point {x: 1.5, y: 69};

        // implementation of generic methods for structs

        // We have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>. By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type
        impl <T, U> Point<T, U> 
        {
            fn x(&self) -> &T
            {
                &self.x
            }

            fn y(&self) -> &U
            {
                &self.y
            }
        }

        // Using generics does not slow down your programme at runtime
        // (it does make compilation longer though)
        // We can define methods only for some types
        impl Point<f32, f32>
        {
            // below function will be avaialbe only for Point<T, U> where 
            // T = U = f32
            fn dist_from_origin(&self) -> f32
            {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
            
    }

    // TRAITS
    {
        // A trait defines the functionality a particular type has and can share with other types
        // We can use TRAIT BOUNDS to specify that a generic type can be ANY TYPE that HAS CERTAIN BEHAVIOR
        
        // Traits are similar to a feature often called interfaces in other languages, although with some differences


        // Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose

        // implementing traits
        // - trait name: Summary
        // - trait func: summarize --> every struct that will have trait Summary
        //                             will have to have 'summarize' method impl
        pub trait Summary
        {
            // we declare method signatures, we can also add default impl of it 
            // fn summarize(&self) -> String;
            fn summarize(&self) -> String
            {
                format!("(Read more), author: {}", self.summarize_author())
            }

            // Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation. 
            fn summarize_author(&self) -> String;
        }

        pub struct NewsArticle
        {
            pub headline: String,
            pub location: String,
            pub author: String,
            pub content: String
        }

        // To implement a trait for given struct:
        // -- after impl we put trait name
        // -- then we put 'for struct_name' since we want to impl it for struct 
        impl Summary for NewsArticle
        {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }

            fn summarize_author(&self) -> String {
                format!("@{}", self.author)
            }
        }
        // if we want to use default implementation of some of traits methods
        // we just don't impl them in our impl, in this case we would do it 
        // like that:
        // impl Summary for NewsArticle {}


        pub struct SocialPost 
        {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub repost: bool,
        }

        impl Summary for SocialPost
        {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }

            fn summarize_author(&self) -> String {
                format!("SocialPost auth: @{}", self.username)
            }
        }


        
        let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        };

        println!("1 new post: {}", post.summarize());
    }

    // Traits as PARAMETERS
    {
        pub trait Summary
        {
            fn summarize(&self) -> String
            {
                format!("(Read more), author: {}", self.summarize_author())
            }

            fn summarize_author(&self) -> String;
        }

        // Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. So we accept any type that implements Summary trait
        pub fn notfiy(item: &impl Summary)  // SYNTAX SUGAR
        {
            println!("Beaking news ! {}", item.summarize());
        }
        
        // Full syntax - trait bound.
        // --> syntax sugar good for easy cases
        // --> trait bound is good for both cases (I like this syntax better)
        fn notify_full_syntax<T: Summary>(item: &T)
        {
            println!("Beaking news ! (full syntax) {}", item.summarize());
        }

        // we allow different types, but thry have to impl Summary trait
        // pub fn notify(item1: &impl Summary, item2: &impl Summary)

        // both items have to be the same type that implements Summary trait
        // pub fn notify<T: Summary>(item1: &T, item2: &T)


        // Specifying more traits in simple syntax
        fn notify_mult_simple(item: &(impl Summary + Display))
        {

        }

        fn notify_mult_comples<T: Summary + Display>(item: &T)
        {

        }
    }

    // WHERE clause
    {
        // When we have many traits both syntaxes might become unreadable, i.e.:
        fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
        {
            69
        }

        // Thus we can use WHERE clause
        fn some_func_where<T, U>(t: &T, u: &U) -> i32
        where 
            T: Display + Clone,
            U: Clone + Debug
        {
            // instead of writing traits inside <> we write them in where clause
            // thus readability of code gets much better
            2137
        }

        // Returning types that impl traits, 
        fn returns_trait_type() -> impl Display
        {
            String::from("some value")
        }

        // you can only use impl Trait if you’re RETURNING A SINGLE TYPE
        // code below will give an error, since we return two types
        // fn returns_summarizable(switch: bool) -> impl Display 
        // {
        //     if switch {
        //         69
        //     }
        //     else
        //     {
        //         String::from("some str")
        //     }
        // }
    }

    // Trait bounds to Conditionally implement Methods
    {
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        // in the next impl block, Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }


        // We can also conditionally implement a trait for any type that implements another trait
        // We want to implement a trait 'ToString' for type 'T', but only for
        // type 'T' that has alreadg 'Display' trait implemented
        //
        // impl<T: Display> ToString for T {...}
        
    }
}
