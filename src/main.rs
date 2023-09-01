// welcome! I'm going to try and copy useful snippets of code here to make future projects easier to start

// compiler flags to allow certain naughty things during development iteration
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// imports! kinda like #include in C/C++
use serde::Deserialize;
use toml::{Table, Value};
use std::fs;

// global variable initialization
// must be static or const
// must be type annotated
const SMALL_INTEGER: u8 = 5;
const BIG_INTEGER: u32 = 123456;

// &str is used for immutible, fixed length strings
const CONSTANT_STRING: &str = "Hello, world!"; 

// we also have static which is more of a global variable, 
// and has a location in memory rather than being inlined during compile time
// probably won't use these very much
static STATIC_FLOAT: f32 = 3.14159;

//TODO: add toml parsing example

fn main() {
    println!("Hello, world!");
}
