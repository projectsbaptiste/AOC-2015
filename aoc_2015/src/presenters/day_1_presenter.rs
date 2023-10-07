//! # presenters
//!
//! `presenters` generique presenter implementations
//!
use crate::domains::day_1::{day_1, day_1_part_2};
use crate::repositories::day_1_entries::Day1PublicEntities;
use crate::utils::file_tools::get_content_file;

pub struct Day1RestPublicAPI {
    entrie: String,
}
trait Day1PublicAPI {
    fn new() -> Self;
    fn launch_day_1(&mut self, entrie: String) -> i32;
    fn start_day_1_real_input(&mut self) -> i32;
    fn start_day_1_part_2_real_input(&mut self) -> i32;
}

fn find_day_result(day_id: String, part: String) -> i32 {
    match (day_id.as_str(), part.as_str()) {
        ("1", "1") => Day1RestPublicAPI::new().start_day_1_real_input(),
        ("1", "2") => Day1RestPublicAPI::new().start_day_1_part_2_real_input(),
        _ => -1,
    }
}

impl Day1RestPublicAPI {
    pub fn new() -> Day1RestPublicAPI {
        Day1RestPublicAPI {
            entrie: String::new(),
        }
    }
    pub fn launch_day_1(&mut self, entrie: String) -> i32 {
        self.set_entrie(entrie);
        day_1(self.entrie.to_string())
    }
    pub fn launch_day_1_from_file(&mut self, entrie: String) -> i32 {
        self.set_entrie(entrie);
        let contents = get_content_file(&self.entrie);
        day_1(contents)
    }
    pub fn day_1_part_2(&self, entrie: String) -> i32 {
        day_1_part_2(entrie)
    }
    pub fn launch_day(&mut self, entrie: String, day_number: i32, part_number: i32) -> i32 {
        self.set_entrie(entrie);
        day_1(self.entrie.to_string())
    }

    /*
    fn day_1_part_2_from_file(&mut self, entrie: String) -> i32 {
        self.set_entrie(entrie);
        let contents = get_content_file(&self.entrie);
        day_1_part_2(contents)
    }
    */
    fn set_entrie(&mut self, entrie: String) {
        self.entrie = entrie;
    }

    pub fn start_day_1_real_input(&mut self) -> i32 {
        self.set_entrie(Day1PublicEntities::get_data_day_1_real_data());
        self.launch_day_1(self.entrie.to_string())
    }
    pub fn start_day_1_part_2_real_input(&mut self) -> i32 {
        let data = Day1PublicEntities::get_data_day_1_real_data();
        self.set_entrie(data);
        self.day_1_part_2(self.entrie.to_string())
    }
}

impl Day1PublicAPI for Day1RestPublicAPI {
    fn new() -> Day1RestPublicAPI {
        Day1RestPublicAPI::new()
    }
    fn launch_day_1(&mut self, entrie: String) -> i32 {
        Day1RestPublicAPI::launch_day_1(self, entrie)
    }
    fn start_day_1_real_input(&mut self) -> i32 {
        Day1RestPublicAPI::start_day_1_real_input(self)
    }
    fn start_day_1_part_2_real_input(&mut self) -> i32 {
        Day1RestPublicAPI::start_day_1_part_2_real_input(self)
    }
}
