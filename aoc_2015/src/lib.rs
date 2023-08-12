//! AOC is a set of exercises to practise solving simple problems.
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
pub mod domains;
pub mod repositories;
pub struct Day1PublicAPI {
    entrie: String,
}

impl Day1PublicAPI {
    pub fn new(entrie: String) -> Day1PublicAPI {
        Day1PublicAPI {
            entrie: entrie.to_string(),
        }
    }
    pub fn launch_day_1(&self) -> i32 {
        domains::day_1::day_1(self.entrie.to_string())
    }
    pub fn launch_day_1_from_file(&self) -> i32 {
        let contents = repositories::file_tools::get_content_file(&self.entrie);
        domains::day_1::day_1(contents)
    }
    pub fn day_1_part_2(&self) -> i32 {
        domains::day_1::day_1_part_2(self.entrie.to_string())
    }
    pub fn day_1_part_2_from_file(&self) -> i32 {
        let contents = repositories::file_tools::get_content_file(&self.entrie);
        domains::day_1::day_1_part_2(contents)
    }
}
