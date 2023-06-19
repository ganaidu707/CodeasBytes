/*
An enum is a type with a closed set of allowed values
E.g. a "Flight" enum might allow Economy,Business,First

You can define a simple enum type as follows:
Specify the enum type, starting with a capital
Specify allowed values ("variants"), also starting with capitals

Enums are extremely important and widely used in Rust
Both in the Rust library, and in the code that we write ourselves
*/
#![allow(dead_code)]

enum Colour {
    Red,
    Green,
    Blue
}

fn demo_simple_enums() {

    println!("Demo simple enums");

    let c = Colour::Blue;   
    match c {
        Colour::Red   => println!("coch"),
        _             => println!("who cares")
    }
}

enum HouseLocation {
    Number(i32),
    Name(String),
    Unknown
}

fn demo_enum_with_data() {

    println!("\nDemo enums with data");

    let h = HouseLocation::Unknown;
    match h {
        HouseLocation::Number(n) => println!("You live in house number {}", n),
        HouseLocation::Name(s)   => println!("You live in a house named {}", s),
        HouseLocation::Unknown   => println!("Your house location is unknown"),
    }
    println!("Btw the size of HouseLocation is {} bytes", std::mem::size_of::<HouseLocation>());
}

fn demo_using_option_enum() {
    
    println!("\nDemo using the Option<T> enum");
    
    // Rust defines a standard enum Option<T> as follows:
    // enum Option<T> {
    //    Some(T),
    //    None
    // }

    let favnum: Option<i32>;

    // Uncomment one of the following statements.
    //favnum = Option::Some(3);
    favnum = Option::None;

    match favnum {
        Some(n) => println!("Your favourite number is {}, good choice", n),
        None    => println!("Dude you don't have a favourite number... What????!")
    }

    // You can use unwrap_or() to extract Some value from an Option,
    // or use a fallback value if None.
    println!("Your fav num  name is {}", favnum.unwrap_or(42));
}






fn demo_using_result_enum(s: String) {
    
    println!("\nDemo using the Result<T, E> enum");

    // Rust defines a standard enum Result<T, Err> as follows:
    // enum Result<T, E> {
    //    Ok(T),
    //    Err(E)
    // }
   
    match s.parse::<i32>() {
        Ok(v)  => println!("Parsed String as i32: {}", v),
        Err(e) => println!("Error parsing String as i32: {}", e),
    }

    // You can use unwrap_or() to extract Ok value from a Result, or use a fallback value if Err.
    let good_str = String::from("world");
    println!("Parsed String as i32: {}", good_str.parse::<i32>().unwrap_or(-1));

    // You can use expect() to specify a panic error message if Result is Err.
    let bad_str = String::from("nineteen sixty four");
    println!("Parsed String as i32: {}", bad_str.parse::<i32>().expect("Invalid year, baby"));
}

fn main() {
    demo_simple_enums();
    demo_enum_with_data();
    demo_using_option_enum();
    demo_using_result_enum(String::from("hello"));
}