#![allow(warnings, unused)]
use clearscreen::clear;
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
            clear();
            println!("{:?}", path_);
        }
        if meta.is_dir() && path_.to_str().unwrap().to_string().contains("node_modules") == false {
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
    let mut counter = 0;
    let files = get_files(path.clone());
    for i in &files {
        let mut compatable: bool = match fs::read_to_string(&i) {
            Ok(file) => true,
            Err(error) => false,
        };
        if get_file_size(&i) < 5000 {
            if compatable {
                fileHash.insert(i.clone(), digest(fs::read_to_string(&i).unwrap()));
            } else {
                let bytes = fs::read(&i).unwrap();
                fileHash.insert(i.clone(), sha256::digest_bytes(&bytes));
            }
        } else {
            bigFiles.push(i.clone());
        }
        clear();
        println!("{} / {}", counter, files.len());
        counter += 1;
    }
    return (fileHash, bigFiles);
}

fn get_file_size(path: &PathBuf) -> u64 {
    let meta = metadata(path).unwrap();
    return meta.len();
}

fn get_folder_info(path: PathBuf) -> HashMap<String, (i32, u64)> {
    let files = get_files(path);
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
            " ".repeat(15 - key.len()),
            value.0,
            " ".repeat(4 - value.0.to_string().len()),
            (value.1 / 1000000)
        );
    }
}

fn main() {
    let mut path: PathBuf = PathBuf::new();
    let mut path_string = String::new();
    println!("Enter path :>");
    stdin()
        .read_line(&mut path_string)
        .expect("failed to read the line");
    path.push(&path_string[..path_string.len() - 1]);
    // let mut hashes: Vec<String> = Vec::new();
    // let mut files: Vec<PathBuf> = Vec::new();
    // for (file, hash) in get_hash_obj(path).0 {
    //     if hashes.contains(&hash) {
    //         println!("{:?}", file);
    //     } else {
    //         hashes.push(hash);
    //     }
    // }
}
