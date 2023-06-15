/*Rust has the following looping constructs:
loop  -  infinite loop, similar to for(;;) in other languages
while -  loop while a condition is true
for   -  loop over a collection*/

/*fn demo_loops() {

  /*println!("\nUsing an infinite loop");
  loop {
    println!("This loop will go on forever. Hit Ctrl-C to stop me!");
  }*/

  /*println!("\nUsing a while loop");
  let mut i = 0;
  while i < 10 {
      println!("{}", i);
      i += 1;
  }*/

  /*println!("\nUsing a for loop over a range");
  println!("\ninclusive lower bound, exclusive upper bound");
  for i in 0..10 {
      println!("{}", i);
  }

  println!("\ninclusive lower bound, inclusive upper bound");
  for i in 0..=10 {
      println!("{}", i);
  }*/

  /*println!("\nUsing a for loop over an array");
  let arr = [99, 55, 95, 100, 82];
  for elem in arr {
    println!("{}", elem);
  }*/
}*/

/*break and continue are similar to other languages
break    -  exit a loop immediately
continue -  terminate current iteration and start next one

You can break/continue nested loops
Label a loop, e.g.,  'outer:  loop
Then break/continue via syntax   break  'outer;*/

fn demo_loops_break_continue() {

  /*println!("\nDemo using break and continue");
  
  let arr = [99, 45, 85, 100, 82];
  for elem in arr {
    if elem == 100 {
        println!("Found 100, so break out of loop completely");
        break;
    } 
    if elem < 50 {
        println!("Found value less than 50, continue to next iteration");
        continue;
    }
    println!("{}", elem);
  }*/

  'outer: loop {
    println!("Entered the outer loop");
    loop {
        println!("Entered the inner loop");
        break 'outer;  // Break the outer loop.
    }
    println!("This point will never be reached in this example");
  }
  println!("Exited the outer loop");

  println!("The End!");
}

fn main() {
  //demo_loops();
  demo_loops_break_continue();
}
