//! # repositories
//!
//! `repositories` is adapter get data from file
//!
//!
use crate::utils::file_tools::get_content_file;

pub struct Day1PublicEntities {
    data: String,
}
impl Day1PublicEntities {
    pub fn new() -> Day1PublicEntities {
        let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
        Day1PublicEntities {
            data: get_content_file(file_path),
        }
    }
    pub fn get_data_day_1_real_data() -> String {
        let file_path = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
        get_content_file(file_path)
    }
}
