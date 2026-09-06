// #![feature(f16_and_f128)]
#![allow(unused)]
fn main(){
    let guess: u32 = "42".parse().expect("Not a number!"); // Rust is a statically typed language.
    println!("{guess}");


    // --------------- scalar types ---------------------

    
    // integers, floating-point, numbers, booleans, characters

    // ================ Integer type ====================
    
    /*
        Integer types: 8-bit, 16-bit, 32-bit, 64-bit, 128-bit -> i: signed, u: unsigned

        Architecture-dependent: isize, usize.

        note: i{-, +}, u{+}
    */

    let integer: i64 = -123; // by deafult rust uses i32 (numeric range and processing speed)

    println!("{integer}");

    // ================= Floating point type =====================

    /*
        float types: 32-bit, 64-bit -> f32, f64

        Default type is f64 (offer same speed as 32-bit and much higher precision)

        note: no native f8 primitive type built into Rust(but we can use through extenal libs). f16 remains an unstable/nightly-only feature under the tracking issue #![feature(f16_and_f128)]
        
    */

    let x = 3.14; // default to f64
    // let y: f16 = 3.14_f16; // f16
    
    // ================= Boolean type =======================

    let t = true;

    let f: bool = false; // with explicit type annotation

    // =================== Character type =====================


    let c = 'z'; //  It is always 4 bytes (32 bits)
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // ----------------- Compound types ------------------
    // Compound types can group multiple values into one type.

    
    // ================== Tuple types ======================

    /*
    
    A tuple is a general way of grouping together a number of values with a variety of types into one.

     - fixed size.
     - wrapped in parentheses () comma seperated.
     - accesss elements using dot . notation.
     - allocated on stack.

     -- De-structuring: It allows you to break apart compound types (like tuples, arrays, structs, and enums) and bind their internal pieces to individual variables in a single step.
     
     
    */

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0); // println! macro only allow plain var names. (use positional argument)

    let (k, l, m) = tup; // de-structuring

    // ================ Array type ===================

    /*
    
    An array is a collection of multiple values of the same type, allocated continuously on the stack.

    - fixed size.
    - Values are wrapped in square brackets [].
    - allocated in stack.

    -- Indexing/element access: access elements of an array using indexing ( single chunk of memory of a known, fixed size)

    */
    let arr: [i32; 4]; // empty array init on stack.
    let a = [1, 2, 3, 4, 5];


    println!("{}", a[0]); // array indexing

    // trick: let i = [3; 4]; -> [3, 3, 3, 3]

}
