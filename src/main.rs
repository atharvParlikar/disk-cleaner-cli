#![allow(warnings, unused)]
use clearscreen::clear;
use core::panic;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    Command,
};
use sha256::digest;
use std::fs::{self, metadata};
use std::io::stdin;
use std::path::{self, PathBuf};
use std::{collections::HashMap, str::Split};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

fn extract_dirname(path: PathBuf) -> String {
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

fn get_files(path: PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for x in fs::read_dir(path).unwrap() {
        let path_ = x.unwrap().path();
        let meta = metadata(&path_).unwrap();
        let avoid: Vec<String> = vec!["node_modules".to_string(), "target".to_string()];
        if meta.is_file() {
            files.push(path_.clone());
        }

        let path_str = path_.clone().to_str().unwrap().to_string();

        if meta.is_dir() && false == false {
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
    path.push(path_str);
    print_extension_map(&get_folder_info(path));
}

fn main() -> Result<(), io::Error> {
    // enable_raw_mode()?;
    // let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default().title("Block").borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;
    // thread::sleep(Duration::from_secs(5));
    // // restore the terminal
    // disable_raw_mode();
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // );

    while true {
        println!("enter command :>");
        let mut command = "".to_string();
        stdin().read_line(&mut command);
        command.pop();
        command = command.trim().to_string();
        match command.as_str() {
            "exit" => break,
            "scan" => scan(),
            "clear" => clear().unwrap(),
            "test" => {
                let mut path_ = PathBuf::new();
                path_.push("/hola/bitch");
                println!("{}", extract_dirname(path_));
            }
            _ => println!("please enter a vlid command"),
        }
    }
    Ok(())
}
