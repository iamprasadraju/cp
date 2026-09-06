#![allow(unused)]

// =============== Functions ================

/* function is a self-contained block of code that performs a specific task. It allows you to break your program into smaller, reusable, and organized chunks.

    - snake_cases style for func
    - Function declaration order not required as c. (Rust compiler scans your entire code structure first to build a "map" (an Abstract Syntax Tree) of all your types and functions before it begins evaluating how they interact)
    - 

*/

fn main(){
    println!("Hello, Rust!");
    args(5);
    another_fn();

    let mut x = expression(1);
    println!("returns {}", x);

}


fn another_fn(){
    println!("Just do it!");
}

// ================ Parameters/args ====================

/* which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters.

    - args/para requires type
    
*/

fn args(x: i32){
    println!("The value of x: {x}");
}

// ================= Statements and Expressions ================

/* Function bodies are made up of a series of statements optionally ending in an expression.

    - statements: Performs an action but does not return a value.
        - ends with ;
        - ex: let x = 5;
    
    - Expressions: returns a value.
        - don't end with ;
        - ex: 5 + 5
*/

fn statement(){
    let x = 5; // statement
    println!("{}", x);
}

fn expression(x: i32) -> i32{
    x + x // expression
}
