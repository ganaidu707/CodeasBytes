/*
"If-tests" are similar to most other languages,
    but note the following points:
    1. Test expression must be bool
    2. No need for parens around test expression
    3. Blocks must be enclosed in {}
    4. You can use if/else as an expression, similar to ?: in C++ etc.
*/
fn demo_if() {
    let age = 58;
    if age > 50 {
        println!("You are old");
    }

    let height = 1.67;
    if height > 1.8 {
        println!("You are tall");
    }
    else {
        println!("You are not so tall");
    }

    let swans_games = 200;
    if swans_games > 300 {
        println!("You are a very loyal fan, we appreciate it dude");
    }
    else if swans_games > 100 {
        println!("You are a discerning fan");
    }
    else {
        println!("You are quite a new fan, welcome buddy");
    }

    let message = if age > 50 {"Hi oldie"} else {"Hi newbie"};
    println!("{}", message);
}

/* "match" is similar to switch in other languages,
    but is much more powerful. You can match on:
    Specific values
    Ranges
    Multiple values
    Conditions
*/
fn demo_match() {

    let num = 100;
    
    println!("\nUsing a match to test an expression against patterns");
    match num {
        100 => println!("A hundred"),
        200 => println!("Two hundred"),
        _   => println!("Who cares")
    }

    match num {
        25..=50  => println!("25 to 50"),
        51..=100 => println!("51 to 100"),
        _        => println!("Who cares")
    }

    match num {
        25 | 50 | 75  => println!("25, 50, or 75"),
        100 | 200     => println!("100 or 200"),
        _             => println!("Who cares")
    }

    match num {
        x if x < 50  => println!("Less than 50"),
        x if x == 75 => println!("75"),
        _             => println!("Who cares (could be 100 maybe)")
    }
}

fn main() {
    demo_if();
    demo_match();
}