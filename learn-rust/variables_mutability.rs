#![allow(unused)]
fn main(){
    
    // let x; // By default, variables in Rust are immutable
    let mut x;
    x = 5;
    println!("{x}");

    x = 6;
    println!("{x}");

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing
    let y = 6; // creates a variable binding
    
    let mut y = y;
    println!("{y}");

    y = 7;   
    println!("{y}");


    // constant

    // const can never be mutable
    // const requires a type
    
    const SECONDS_PER_MINUTE: u32 = 60; // defines a compile-time constant
}
