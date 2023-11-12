//! # presenters
//!
//! `presenters` generique presenter implementations
//!
use crate::domains::day_1::{day_1, day_1_part_2};
use crate::utils::file_tools::get_content_file;
pub struct DayRestPublicAPI {
    day_number: i32,
    day_part: i32,
    day_data: Vec<String>,
    day_result: String,
}

fn find_day_result(day_id: i32, part: i32, day_data: Vec<String>) -> i32 {
    match (day_id.to_string().as_ref(), part.to_string().as_ref()) {
        ("1", "1") => day_1(day_data.get(0).unwrap().to_string()),
        ("1", "2") => day_1_part_2(day_data.get(0).unwrap().to_string()),
        _ => -1,
    }
}
impl DayRestPublicAPI {
    pub fn new(day_number: i32, day_part: i32, day_data: Vec<String>) -> DayRestPublicAPI {
        DayRestPublicAPI {
            day_number: day_number,
            day_part: day_part,
            day_data: day_data,
            day_result: String::new(),
        }
    }
    pub fn launch_day_X_part_Y(&mut self, day_data: Vec<String>) -> i32 {
        self.day_result = find_day_result(self.day_number, self.day_part, day_data).to_string();
        self.day_result.parse().unwrap()
    }
    pub fn launch_day_X_part_Y_from_file(&mut self, day_data: Vec<String>) -> i32 {
        let contents = get_content_file(&self.day_data.get(0).unwrap()).to_string();
        self.day_result = day_1(contents).to_string();
        self.day_result.trim().parse().unwrap()
    }
}
