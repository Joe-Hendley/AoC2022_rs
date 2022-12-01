use std::{
    fs::read_to_string,
    io::{self},
};

const FILE_PATH: &str = "input/";

fn build_input_string(id: i32) -> String {
    return FILE_PATH.to_owned() + &id.to_string() + ".txt";
}

fn build_test_input_string(id: i32) -> String {
    return FILE_PATH.to_owned() + &id.to_string() + "_test.txt";
}

fn extract_input(filepath: &str) -> io::Result<String> {
    read_to_string(filepath)
}

pub fn get_test_input(id: i32) -> String {
    let filepath = build_test_input_string(id);
    let _result = match extract_input(&filepath) {
        Ok(test_input) => return test_input,
        Err(_error) => {
            print!("couldn't find test input at: {}", filepath);
            return String::from("");
        }
    };
}

pub fn get_input(id: i32) -> String {
    let filepath = build_input_string(id);
    let _result = match extract_input(&filepath) {
        Ok(input) => return input,
        Err(_error) => {
            print!("couldn't find input at: {}", filepath);
            return String::from("");
        }
    };
}
