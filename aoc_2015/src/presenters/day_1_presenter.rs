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

impl Day1RestPublicAPI {
    pub fn new() -> Day1RestPublicAPI {
        Day1RestPublicAPI {
            entrie: "".to_string(),
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
    // fn day_1_part_2(&self) -> i32 {
    //     day_1_part_2(self.entrie.to_string())
    // }
    fn day_1_part_2_from_file(&self) -> i32 {
        let contents = get_content_file(&self.entrie);
        day_1_part_2(contents)
    }
    fn set_entrie(&mut self, entrie: String) {
        self.entrie = entrie;
    }

    pub fn start_day_1_real_input(&mut self) -> i32 {
        let data = Day1PublicEntities::get_data_day_1_real_data();
        self.set_entrie(data);
        self.day_1_part_2_from_file()
    }
    pub fn start_day_1_part_2_real_input(&mut self) -> i32 {
        let data = Day1PublicEntities::get_data_day_1_real_data();
        self.set_entrie(data);
        self.day_1_part_2_from_file()
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
