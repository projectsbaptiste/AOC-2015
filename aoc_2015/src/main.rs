//! MAIN AOC is a set of exercises to practise solving simple problems.
//!
//! aoc_2015 is a project to solve these problems
//!
//! # Goal
//!  
//! I have tried to implement several techniques to train myself. TDD, a hexagonal architecture from the 3rd problem.  And TCR from the 5th problem.
//!
//! # Baby step by <https://www.youtube.com/watch?v=xI_iN1HNweI>
//!
//!
//! 1 Extract method
//!
//! 2 Extract parameter in function or method
//!
//! 3 Extract Constant
//!
//! 4 Add a test
//!
//! 5 Add a case to an existing switch or if with a dead simple implementation
//!
//! 6 Replace an expression with a function call or an more complex expression
//!
//! 7 Undo the last step
//!

//use actix_web::{App, HttpServer};
//use aoc_2015::Day1PublicAPI;

pub mod domains;
pub mod presenters;
pub mod repositories;
pub mod utils;
use presenters::day_1_ui::start_server;

// not need async ?
fn main() {
    let _ = start_server();
}
