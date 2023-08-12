use aoc_2015::Day1PublicAPI;
#[test]
fn day_1_with_one_parnetesis_entrie_should_return_floor_1() {
    let exemple = "(";
    let day_1_test = Day1PublicAPI::new(exemple.to_string());
    assert_eq!(day_1_test.launch_day_1(), 1);
}

#[test]
#[ignore = "only on local in my computer"]
fn day_1_with_file_absolute_path_should_return_floor_1() {
    let exemple = "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt";
    let day_1_test = Day1PublicAPI::new(exemple.to_string());
    assert_eq!(day_1_test.launch_day_1_from_file(), 1);
}

#[test]
fn day_1_with_file_relative_path_should_return_floor_1() {
    let exemple = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
    let day_1_test = Day1PublicAPI::new(exemple.to_string());
    assert_eq!(280, day_1_test.launch_day_1_from_file());
}

/// todo: fix should retourn None ?
#[test]
fn day_1_with_error_path_should_return_floor_0() {
    let exemple = "t";
    let day_1_test = Day1PublicAPI::new(exemple.to_string());
    assert_eq!(0, day_1_test.launch_day_1_from_file());
}

#[test]
fn day_1_part_2_should_return_position_1797() {
    let exemple = "../aoc_2015_inputs/day_1/real_input_from_site.txt";
    let day_1_test = Day1PublicAPI::new(exemple.to_string());
    assert_eq!(1797, day_1_test.day_1_part_2_from_file());
}
