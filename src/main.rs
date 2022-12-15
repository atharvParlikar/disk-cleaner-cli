#![allow(warnings, unused)]
use clearscreen::clear;
use core::panic;
use sha256::{digest, digest_file};
use std::io::Stdout;
use std::{collections::HashMap, str::Split};
use std::{
    fs::File,
    io::{stdin, stdout, BufReader},
};
use std::{
    fs::{self, metadata},
    ops::Not,
};
use std::{io, thread};
use std::{
    io::Read,
    path::{self, PathBuf},
};
use std::{
    io::Write,
    time::{Duration, Instant},
};

static mut TOTAL: i32 = 0;

fn extract_dirname(path: &PathBuf) -> String {
    let path_str = path.to_str().unwrap().to_string();
    let mut dirname: String = "".to_string();
    let mut left_shift = 1;
    if path_str.chars().nth(path_str.len() - 1).unwrap() == '/' {
        left_shift += 1;
    }
    let mut index = path_str.len() - left_shift;
    while path_str.chars().nth(index).unwrap() != '/' {
        dirname.push(path_str.chars().nth(index).unwrap());
        index -= 1;
    }
    return dirname.chars().rev().collect::<String>();
}

fn get_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for x in fs::read_dir(path).unwrap() {
        let path_ = x.unwrap().path();
        let meta = metadata(&path_).unwrap();
        let avoid: Vec<String> = vec![
            "node_modules".to_string(),
            "target".to_string(),
            ".git".to_string(),
        ];
        if meta.is_file() {
            files.push(path_.clone());
        }
        let path_str = path_.clone().to_str().unwrap().to_string();
        if meta.is_dir() && avoid.contains(&extract_dirname(&path_)).not() {
            let files_ = get_files(&path_);
            for file in files_ {
                files.push(file);
            }
        }
    }
    return files;
}

fn get_big_file_hash(path: &PathBuf) -> String {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1000];
    reader.read(&mut buffer);
    return sha256::digest_bytes(&buffer);
}

fn get_hash_obj(path: PathBuf) -> HashMap<PathBuf, String> {
    let mut file_hash: HashMap<PathBuf, String> = HashMap::new();
    let files = get_files(&path);
    for i in files {
        if get_file_size(&i) < 1000 {
            let mut compatable: bool = match fs::read_to_string(&i) {
                Ok(file) => true,
                Err(error) => false,
            };
            if compatable {
                file_hash.insert(i.clone(), digest_file(&i).unwrap());
            } else {
                let bytes = fs::read(&i).unwrap();
                file_hash.insert(i.clone(), sha256::digest_bytes(&bytes));
            }
        } else {
            file_hash.insert(i.clone(), get_big_file_hash(&i));
        }
    }
    return file_hash;
}

fn get_file_size(path: &PathBuf) -> u64 {
    let meta = metadata(path).unwrap();
    return meta.len();
}

fn get_folder_info(path: PathBuf) -> HashMap<String, (i32, u64)> {
    // in the returning touple (i32, u64) i32 is the number of files for that extension
    // and u64 is the total size of all files combined for that extension (in bytes)
    let files = get_files(&path);
    let mut extension_map: HashMap<String, (i32, u64)> = HashMap::new();
    for i in files {
        let file_len = i.metadata().unwrap().len();
        match i.extension() {
            Some(x) => {
                let file_extension: String = x.to_str().unwrap().to_string();
                if extension_map.contains_key(&file_extension) {
                    let new_val = (
                        extension_map[&file_extension].0 + 1,
                        extension_map[&file_extension].1 + file_len,
                    );
                    *extension_map.get_mut(&file_extension).unwrap() = new_val;
                } else {
                    extension_map.insert(file_extension, (1, file_len));
                }
            }
            None => {
                if extension_map.contains_key(&"NoneType".to_string()) {
                    let new_val = (
                        extension_map[&"NoneType".to_string()].0 + 1,
                        extension_map[&"NoneType".to_string()].1 + file_len,
                    );
                    *extension_map.get_mut(&"NoneType".to_string()).unwrap() = new_val;
                } else {
                    extension_map.insert("NoneType".to_string(), (1, 0));
                }
            }
        }
    }
    return extension_map;
}

fn print_extension_map(extension_map: &HashMap<String, (i32, u64)>) {
    for (key, value) in extension_map {
        println!(
            "{}{}{}{} -> {}Mb",
            key,
            " ".repeat(20 - key.len()),
            value.0,
            " ".repeat(4 - value.0.to_string().len()),
            (value.1 / 1000000)
        );
    }
}

fn find_duplicate(file_hashmap: HashMap<PathBuf, String>) -> Vec<PathBuf> {
    let mut hashes: Vec<String> = Vec::new();
    let mut duplicate_paths: Vec<PathBuf> = Vec::new();
    for (file, hash) in file_hashmap {
        if hashes.contains(&hash) {
            duplicate_paths.push(file);
        } else {
            hashes.push(hash);
        }
    }
    return duplicate_paths;
}

fn scan() {
    println!("enter path:");
    let mut path_str = "".to_string();
    stdin().read_line(&mut path_str);
    path_str.pop();
    let mut path = PathBuf::new();
    path.push(path_str.clone());
    let folderinfo = get_folder_info(path.clone());
    print_extension_map(&folderinfo);
    let mut total = 0;
}

fn dup() {
    print!("enter path:");
    stdout().flush();
    let mut path_str = "".to_string();
    stdin().read_line(&mut path_str);
    path_str.pop();
    let mut path = PathBuf::new();
    path.push(path_str);
    let duplicate = find_duplicate(get_hash_obj(path));
    println!("{:?}", duplicate);
}

// /home/atharv/Downloads/Video

fn main() -> Result<(), io::Error> {
    while true {
        print!("enter command :> ");
        stdout().flush();
        let mut command = "".to_string();
        stdin().read_line(&mut command);
        command.pop();
        command = command.trim().to_string();
        match command.as_str() {
            "exit" => break,
            "scan" => scan(),
            "clear" => clear().unwrap(),
            "test" => {
                let start = Instant::now();
                let mut path = PathBuf::new();
                path.push("/home/atharv/Downloads/Video/");

                // expensive computation
                for i in get_files(&path) {
                    println!("{}", get_big_file_hash(&i));
                }

                let duration = start.elapsed();
                println!("time taken =>");
                println!("{:?}", duration);
            }
            "dup" => dup(),
            "getFiles" => {
                let mut path = String::new();
                print!("Enter path: ");
                stdout().flush();
                stdin().read_line(&mut path);
                path.pop();
                let mut path_ = PathBuf::new();
                path_.push(&path);
                println!("{:?}", get_files(&path_));
            }
            _ => println!("please enter a vlid command"),
        }
    }
    Ok(())
}
