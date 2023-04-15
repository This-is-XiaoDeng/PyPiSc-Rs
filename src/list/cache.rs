use std::{
    fs::File,
    io::{Read, Write},
};
// use serde_json::from_str;
use std::env;

pub fn make_cache(data: Vec<String>) -> std::io::Result<()> {
    let working_dir: String = env::temp_dir().display().to_string();
    let mut file = File::create(format!("{}/cache.json", working_dir)).unwrap();
    file.write(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();
    Ok(())
}

pub fn load_cache() -> Vec<String> {
    let working_dir: String = env::temp_dir().display().to_string();
    let mut file = File::open(format!("{}/cache.json", working_dir)).unwrap();
    let mut raw_json = String::new();
    file.read_to_string(&mut raw_json).unwrap();
    serde_json::from_str(raw_json.as_str()).unwrap()
}
