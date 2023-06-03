fn datatypes_techniques() {
    // Rust can infer types.
    /*let a = -12345;
    let b = 3.14;
    let c = 'X';
    println!("\na is {}, b is {}, c is {}", a, b, c);*/

    // Variables are immutable by default.
    /*let d = 0;
    //d = 1;
    println!("d is {}", d);

    // You must use the "mut" qualifier to make a variable mutable.
    let mut e = 0;
    println!("e originally is {}", e);
    e = 1;
    println!("e afterwards is {}", e);*/
 
    // If you don't use a variable, prefix name with _ to avoid compiler warning.
    //let _f = 0;

    // You can cast using the "as" keyword.
    /*let g = 3.99;
    let h = g as i32;
    println!("g is {}, h is {}", g, h);*/

    // Rust enables you to redeclare a variable in the current scope.
    //This is called shadowing and it's quite cool.
    /*let num = String::from("12345");
    println!("num as a string is {}", num);
    let num = 12345;
    println!("num+1 as a number is {}", num + 1);*/

    // You can define compile-time constants. You must specify the type btw.
    const SECONDS_IN_HOUR: i32 = 3_6_0_0;
    const SECONDS_IN_DAY: i32 = 24 * SECONDS_IN_HOUR;
    println!("There are {} seconds in a day", SECONDS_IN_DAY);
}

fn main() {
    datatypes_techniques();
}
