mod domains;
mod repositories;
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
