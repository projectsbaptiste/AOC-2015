use std::fs;
use std::path::Path;

fn check_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    let test = path.canonicalize();
    match test {
        Ok(_) => return true,
        Err(_) => return false,
    };
}

fn get_content_file(file_path: &str) -> String {
    println!("In file {}", file_path);
    if (check_file(file_path)) {
        let contents = fs::read_to_string(
            Path::new(file_path)
                .canonicalize()
                .unwrap()
                .as_os_str()
                .to_str()
                .expect("msg"),
        );
        println!("Content {:?}", contents);
        return contents.expect("Problem");
    } else {
        println!("Problem to canonicalize file");
        return "Problem to canonicalize file".to_string();
    }
}

mod tests {
    use super::*;
    #[test]
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
