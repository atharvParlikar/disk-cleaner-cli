use sha256::digest;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

fn main() {
    let dir_contents = fs::read_dir("./src").unwrap();
    let mut file_hash_map: HashMap<PathBuf, String> = HashMap::new();
    for x in dir_contents {
        let path = x.unwrap().path();
        let file_hash = digest(fs::read_to_string(&path).unwrap());
        file_hash_map.insert(path, file_hash);
    }
    println!("\n");
    for (path, hash) in &file_hash_map {
        println!("{:?} : {}", path, hash);
    }
}
