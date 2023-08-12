//! # repositories
//!
//! `repositorues` is a collection of tools its whats the program need to get entries. Its maybe the adapter  fromhexagonal architecture
//!
//! file tools

use std::fs;
use std::path::Path;

/// # check_file
///
///  fonction who check file can be canonicalize . exist  because if I use unwrap on canonulize I have panic If I have no file
///
/// Returns true when all is good false otherside
///
/// # Arguments
///
/// * `file_path` - A string where we got file path work should work on absolute and relative path
///
/// Remarque . is the place at the same place as src
/// # Examples
///
/// ```
/// on the file "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt"  I got an '('
/// exemple 1
/// "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt" -> file path
/// should return true
///
/// exemple 2
/// "../aoc_2015_inputs/day_1/test_one_parenthesis.txt" -> file_path
/// should return true
///
///
/// exemple 3
/// "../aoc_2015_inputs/day_1/unexist.txt" -> file_path
/// should return false
/// ```
fn check_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let test = path.canonicalize();
    match test {
        Ok(_) => return true,
        Err(_) => return false,
    };
}
/// # utils
///
///  fonction who get content file into a string from a file path
///
/// Returns the content of a file into a string or Problem to canonicalize file
///
/// # Arguments
///
/// * `file_path` - A string where we got file path work should work on absolute and relative path
///
/// Remarque . is the place at the same place as src
/// # Examples
///
/// ```
/// on the file "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt"  I got an '('
/// exemple 1
/// "/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015_inputs/day_1/test_one_parenthesis.txt" -> file path
/// should return '('
///
/// exemple 2
/// "../aoc_2015_inputs/day_1/test_one_parenthesis.txt" -> file_path
/// should return 5
///
/// ```
pub fn get_content_file(file_path: &str) -> String {
    println!("In file {}", file_path);
    if check_file(file_path) {
        let contents = fs::read_to_string(
            Path::new(file_path)
                .canonicalize()
                .unwrap()
                .as_os_str()
                .to_str()
                .expect("msg"),
        );
        contents.expect("Problem")
    } else {
        println!("Problem to canonicalize file");

        "Problem to canonicalize file".to_string()
    }
}

mod tests {
    use super::get_content_file;
    #[test]
    #[ignore = "only on local in my computer"]
    fn read_file_this_file() {
        let absolute_path="/home/baptiste/Documents/rust/projects/AOC-2015/aoc_2015/src/repositories/file_tools.rs";
        get_content_file(absolute_path);
    }
    #[test]
    fn read_file_this_file_relative_path() {
        let relative_path = "./Cargo.toml";
        get_content_file(relative_path);
    }

    #[test]
    fn read_file_this_file_doesnt_exist() {
        let relative_path = "./Cargjo.toml";
        get_content_file(relative_path);
    }
}
