use glob::glob_with;
use glob::MatchOptions;
use glob::{Paths, PatternError};
use regex::Regex;
use std::ffi::OsString;
use std::fs::File;
use std::io::Result as IOResult;
use std::io::{BufRead, BufReader, Lines};

pub mod list;
pub mod project;
pub mod sourcefile;
pub mod todo;

use sourcefile::SourceFile;
use todo::Todo;

fn get_paths_from_dir(dir: &str) -> Result<Paths, PatternError> {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    glob_with(dir, options)
}

fn build_file_from_path(paths: Paths) -> Vec<SourceFile> {
    paths
        .filter_map(|glob_res| {
            if let Ok(path) = glob_res {
                if let (Some(name), Some(full_path)) = (path.file_name(), path.to_str()) {
                    if let Some(filename) = name.to_str() {
                        Some(SourceFile::new(
                            String::from(filename),
                            String::from(full_path),
                        ))
                    } else {
                        // Todo: handle Result appropriatly
                        None
                    }
                } else {
                    None
                }
            } else {
                // Todo: Handle GlobError appropriately
                None
            }
        })
        .collect()
}

// Todo: Parallel Extraction
fn extract_todos_from_files(files: Vec<SourceFile>) -> IOResult<Vec<Todo>> {
    let mut todos: Vec<Todo> = vec![];

    for file in files {
        let content = read_lines_of_file(&file)?;
        let fileTodos = extract_todos_from_content(content, file);
        todos.extend(fileTodos);
    }

    Ok(todos)
}

fn read_lines_of_file(file: &SourceFile) -> IOResult<Lines<BufReader<File>>> {
    let f = File::open(&file.path)?;
    let reader = BufReader::new(f).lines();
    Ok(reader)
}

fn extract_todos_from_content(lines: Lines<BufReader<File>>, file: SourceFile) -> Vec<Todo> {
    let mut todos: Vec<Todo> = vec![];
    let todo_regex = Regex::new(r"(?i:todo+:?\s?)(?P<todo>.*$)").unwrap();

    for (lnr, line) in lines.enumerate() {
        for capture in todo_regex.captures_iter(&line.unwrap()) {
            let found_todo = Todo::new(String::from(&capture["todo"]), file.clone(), lnr + 1);
            todos.push(found_todo)
        }
    }

    todos
}

// ~~~~~~~~~~~~~~~~~~~~ TESTS ~~~~~~~~~~~~~~~~~~~~ //
#[cfg(test)]
mod tests {
    use super::SourceFile;
    use super::Todo;
    use std::path::PathBuf;

    #[test]
    fn find_paths_from_dir() {
        for p in super::get_paths_from_dir("env_tests/mod_scan/*.txt").unwrap() {
            if let Ok(path) = p {
                let mut expected_path = PathBuf::new();
                expected_path.push("env_tests");
                expected_path.push("mod_scan");
                expected_path.push("file1");
                expected_path.set_extension("txt");
                assert_eq!(path, expected_path)
            }
        }
    }

    #[test]
    fn create_file_vec_from_path() {
        let test_path = super::get_paths_from_dir("env_tests/mod_scan/*.txt").unwrap();
        let files = super::build_file_from_path(test_path);
        let expected = vec![SourceFile {
            name: String::from("file1.txt"),
            path: String::from("env_tests/mod_scan/file1.txt"),
        }];
        assert_eq!(files, expected)
    }

    #[test]
    fn extract_todo_from_test_file() {
        let test_file = SourceFile::new(
            String::from("file1.txt"),
            String::from("env_tests/mod_scan/file1.txt"),
        );
        let expected_todo = vec![Todo::new(String::from("Test"), test_file.clone(), 1)];

        let todo = super::extract_todos_from_files(vec![test_file]).unwrap();

        assert_eq!(todo, expected_todo)
    }
}
