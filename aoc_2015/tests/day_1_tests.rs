use aoc_2015::presenters::day_1_presenter::Day1RestPublicAPI;
#[test]
fn day_1_with_one_parnetesis_entrie_should_return_floor_1() {
    let exemple = "(".to_string();
    assert_eq!(Day1RestPublicAPI::new().launch_day_1(exemple), 1);
}

#[test]
#[ignore = "only on local in my computer"]
fn day_1_with_file_absolute_path_should_return_floor_1() {
    let exemple = "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt".to_string();
    assert_eq!(Day1RestPublicAPI::new().launch_day_1_from_file(exemple), 1);
}

#[test]
fn day_1_with_file_relative_path_should_return_floor_1() {
    let exemple = "../aoc_2015_inputs/day_1/real_input_from_site.txt".to_string();
    assert_eq!(
        280,
        Day1RestPublicAPI::new().launch_day_1_from_file(exemple)
    );
}

/// todo: fix should retourn None ?
#[test]
fn day_1_with_error_path_should_return_floor_0() {
    let exemple = "t".to_string();
    assert_eq!(0, Day1RestPublicAPI::new().launch_day_1_from_file(exemple));
}

#[test]
fn day_1_part_2_should_return_position_1797() {
    assert_eq!(
        1797,
        Day1RestPublicAPI::new().start_day_1_part_2_real_input()
    );
}
