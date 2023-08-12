//! # domains
//!
//! `domains` is a collection of katas with Santa topic
//!
//! -statement of day 1 problem : <https://adventofcode.com/2015/day/1>

/// # nb of occurence
///
///  nomber of occurence of only a char
fn nb_of_occurence(entrie: String, seek: char) -> i32 {
    return entrie.chars().filter(|c| c == &seek).count() as i32;
}

const UP: char = '(';
const DOWN: char = ')';
/// day_1
///
///  find the floor by instruction
pub fn day_1(entrie: String) -> i32 {
    let direction_up = nb_of_occurence(entrie.clone(), UP);
    let direction_down = nb_of_occurence(entrie.clone(), DOWN);
    return direction_up - direction_down;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_1_should_return_floor_0() {
        let directions_exemple1 = "(())";
        let directions_exemple2 = "()()";
        assert_eq!(0, day_1(directions_exemple1.to_string()));
        assert_eq!(0, day_1(directions_exemple2.to_string()));
    }

    #[test]
    fn day_1_should_return_floor_3() {
        let directions_exemple1 = "(((";
        let directions_exemple2 = "(()(()(";
        let directions_exemple3 = "(()(()(";
        assert_eq!(3, day_1(directions_exemple1.to_string()));
        assert_eq!(3, day_1(directions_exemple2.to_string()));
        assert_eq!(3, day_1(directions_exemple3.to_string()));
    }

    #[test]
    fn day_1_should_return_floor_minus_1() {
        let directions_exemple1 = "())";
        let directions_exemple2 = "))(";
        assert_eq!(-1, day_1(directions_exemple1.to_string()));
        assert_eq!(-1, day_1(directions_exemple2.to_string()));
    }

    #[test]
    fn day_1_should_return_floor_minus_3() {
        let directions_exemple1 = ")))";
        let directions_exemple2 = ")())())";
        assert_eq!(-3, day_1(directions_exemple1.to_string()));
        assert_eq!(-3, day_1(directions_exemple2.to_string()));
    }
}
