use std::cmp;

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
    }
}