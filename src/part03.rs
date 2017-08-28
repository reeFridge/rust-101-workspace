// Rust-101, Part 03: Input
// ========================


// I/O is provided by the module `std::io`, so we first have to import that with `use`.
// We also import the I/O *prelude*, which makes a bunch of commonly used I/O stuff
// directly available.
use std::io::prelude::*;
use std::io;

use std::str::FromStr;

fn read_some_vec<T: FromStr>() -> Vec<T> {
    let mut vec: Vec<T> = Vec::<T>::new();
    let stdin = io::stdin();

    println!("Enter a list of numbers, one per line. End with Ctrl-D (Linux) or Ctrl-Z (Windows).");

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.trim().parse::<T>() {
            Ok(num) => {
                vec.push(num);
            },
            Err(_) => {
                println!("Is not a number you tap");
            },
        }
    }

    vec
}

// For the rest of the code, we just re-use part 02 by importing it with `use`.
use part02::{SomethingOrNothing,Something,Nothing,vec_min};

// If you update your `main.rs` to use part 03, `cargo run` should now ask you for some numbers,
// and tell you the minimum. Neat, isn't it?
pub fn main() {
    let vec = read_some_vec::<f32>();
    let min = vec_min(vec);
    min.print2();
}

// **Exercise 03.1**: Define a trait `Print` to write a generic version of `SomethingOrNothing::print`.
// Implement that trait for `i32`, and change the code above to use it.
// I will again provide a skeleton for this solution. It also shows how to attach bounds to generic
// implementations (just compare it to the `impl` block from the previous exercise).
// You can read this as "For all types `T` satisfying the `Print` trait, I provide an implementation
// for `SomethingOrNothing<T>`".
// 
// Notice that I called the function on `SomethingOrNothing` `print2` to disambiguate from the `print` defined previously.
// 
// *Hint*: There is a macro `print!` for printing without appending a newline.
pub trait Print {
    fn print2(self);
}

impl<T: Print> SomethingOrNothing<T> {
    fn print2(self) {
        print!("Something is: ");
        match self {
            Something(n) => {
                n.print2();
            },
            Nothing => println!("<Nothing>")
        };
    }
}

impl Print for i32 {
    fn print2(self) {
        println!("{}", self);
    }
}

// **Exercise 03.2**: Building on exercise 02.2, implement all the things you need on `f32` to make your
// program work with floating-point numbers.
impl Print for f32 {
    fn print2(self) {
        println!("{}", self);
    }
}
