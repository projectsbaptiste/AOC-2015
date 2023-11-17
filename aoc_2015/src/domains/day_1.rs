//! # domains
//!
//! `domains` is a collection of katas with Santa topic
//!
//! -statement of day 1 problem : <https://adventofcode.com/2015/day/1>

use log::debug;

/// # nb of occurence
///
/// nomber of occurence of a char in a string
/// Returns a nb of occurence of a char present in a string
///
/// # Arguments
///
/// * `entrie` - A string where we search to know occurence of a char
///
/// * `wanted` - The char searched into a string
///
/// # Examples
///
///
/// exemple 1
/// '()))' -> entry
/// '(' -> wanted
/// should return 1
///
/// exemple 2
/// '()))' -> entry
/// ')' -> wanted
/// should return 3
///
///
fn nb_of_occurence(entrie: String, wanted: char) -> i32 {
    debug!("nb_of_occurence entrie {} wanted {}", entrie, wanted);
    return entrie.chars().filter(|c| c == &wanted).count() as i32;
}

const UP: char = '(';
const DOWN: char = ')';

/// # day 1
///
/// Santa is trying to deliver presents in a large apartment building, but he can't find the right floor
/// - the directions he got are a little confusing.
/// He starts on the ground floor (floor 0) and then follows the instructions one character at a time.
/// An opening parenthesis, (, means he should go up one floor, and a closing parenthesis, ), means he should go down one floor.
/// Help Santa to know the floor at the end of these instructions
///
/// Returns the last floor after read instruction
///
/// # Arguments
///
/// * `entrie` - A string where we got a serie of instruction
///
/// # Examples
///
///
/// exemple 1
/// '(())' -> entry or '()()' -> entry
/// should return 0
///
/// exemple 2
/// '(((' -> entry or '(()(()(' -> entry
/// should return 3
///
/// exemple 3
/// '))(((((' -> entry
/// should return 3
///
/// exemple 4
/// '())' -> entry or '))(' -> entry
/// should return -1
///
/// exemple 5
/// ')))' -> entry or ')())())' -> entry
/// should return -3
///
///
pub fn day_1(entrie: String) -> i32 {
    let direction_up = nb_of_occurence(entrie.clone(), UP);
    let direction_down = nb_of_occurence(entrie.clone(), DOWN);
    return direction_up - direction_down;
}
/// # day 1 part 2
///
/// Now, given the same instructions, find the position of the first character that causes him to enter the basement (floor -1).
/// The first character in the instructions has position 1, the second character has position 2, and so on.
///
/// Returns the nomber of instructions to do for return to basementfloor-1
///
/// # Arguments
///
/// * `entrie` - A string where we got a serie of instruction
///
/// # Examples
///
///
/// exemple 1
/// ')' -> entry
/// should return 1
///
/// exemple 2
/// '()())' -> entry
/// should return 5
///
///
pub fn day_1_part_2(entrie: String) -> i32 {
    let mut current_floor = 0;
    let mut cpt = 0;
    let vec_char: Vec<char> = entrie.chars().collect();
    for c in vec_char {
        cpt += 1;
        match c {
            UP => current_floor += 1,
            DOWN => current_floor -= 1,
            _ => panic!("in the entrie we got a char not know"),
        }
        if current_floor == -1 {
            return cpt;
        }
    }
    return 0;
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

    #[test]
    fn day_1_part_2_should_return_position_1() {
        let directions_exemple1 = ")";
        assert_eq!(1, day_1_part_2(directions_exemple1.to_string()));
    }

    #[test]
    fn day_1_part_2_should_return_position_5() {
        let directions_exemple1 = "()())";
        assert_eq!(5, day_1_part_2(directions_exemple1.to_string()));
    }
}
