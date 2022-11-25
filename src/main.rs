#![allow(warnings, unused)]
use sha256::digest;
use std::collections::HashMap;
use std::fs::{self, metadata};
use std::path::PathBuf;

fn print_tree(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    for path in paths {
        let metadata = metadata(&path).unwrap();
        if metadata.is_dir() {
            let dir_contents = fs::read_dir(&path).unwrap();
            let mut new_vec: Vec<PathBuf> = Vec::new();
            for i in dir_contents {
                new_vec.push(i.expect("unable to get path").path());
            }
            print_tree(new_vec);
        }
        if metadata.is_file() {
            println!("{:?}", path);
        }
    }
    let mut test: Vec<PathBuf> = Vec::new();
    test.push(PathBuf::new());
    return test;
}

fn main() {
    let dir_contents = fs::read_dir(".").unwrap();
    let mut new_vec: Vec<PathBuf> = Vec::new();
    for i in fs::read_dir(".").unwrap() {
        new_vec.push(i.unwrap().path());
    }
    print_tree(new_vec);
}
