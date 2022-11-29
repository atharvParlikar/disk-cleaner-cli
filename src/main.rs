#![allow(warnings, unused)]
use core::panic;
use sha256::digest;
use std::collections::HashMap;
use std::fs::{self, metadata};
use std::io::stdin;
use std::path::{self, PathBuf};

fn get_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for x in fs::read_dir(path).unwrap() {
        let path_ = x.unwrap().path();
        let meta = metadata(&path_).unwrap();
        if meta.is_file() {
            files.push(path_.clone());
        }
        if meta.is_dir() {
            let files_ = get_files(path_);
            for file in files_ {
                files.push(file);
            }
        }
    }
    return files;
}

fn get_hash_obj(path: PathBuf) -> (HashMap<PathBuf, String>, Vec<PathBuf>) {
    let mut fileHash: HashMap<PathBuf, String> = HashMap::new();
    let mut bigFiles: Vec<PathBuf> = Vec::new();
    for i in get_files(path.clone()) {
        let mut compatable: bool = match fs::read_to_string(&i) {
            Ok(file) => true,
            Err(error) => false,
        };
        if get_file_size(i.clone()) < 5000 {
            if compatable {
                fileHash.insert(i.clone(), digest(fs::read_to_string(&i).unwrap()));
            } else {
                let bytes = fs::read(&i).unwrap();
                fileHash.insert(i.clone(), sha256::digest_bytes(&bytes));
            }
        } else {
            bigFiles.push(i.clone());
            println!("{:?}", i);
        }
    }
    return (fileHash, bigFiles);
}

fn get_file_size(path: PathBuf) -> u64 {
    let meta = metadata(path).unwrap();
    return meta.len();
}

fn main() {
    let mut path: PathBuf = PathBuf::new();
    let mut path_string = String::new();
    println!("Enter path :>");
    stdin()
        .read_line(&mut path_string)
        .expect("failed to read the line");
    path.push(&path_string[..path_string.len() - 1]);
    for big_file in get_hash_obj(path).1 {
        println!("{:?}", big_file);
    }
}
