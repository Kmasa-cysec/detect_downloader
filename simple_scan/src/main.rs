use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

// "/home/kmasa/seccamp2020/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446")

//const SIGN: &str = "wget http://";
//const SIGN = Regex::new(r"wget http://+;").unwrap();
//
fn check_dir(target_path: &str) -> bool {
    let check = metadata(target_path).unwrap();
    check.is_dir()
}

fn search_dir(target_path: &str) -> u8 {
    let paths = fs::read_dir(target_path).unwrap();
    let mut detected_count_temp: u8 = 0;
    let mut return_detected_count: u8 = 0;
    for path in paths {
        let path_str: &str = &(path.unwrap().path().display()).to_string();
        if check_dir(path_str) {
            search_dir(path_str);
        } else {
            println!("Scan {}", path_str);
            return_detected_count = simple_scan_file(path_str);
        }
        detected_count_temp = detected_count_temp + return_detected_count;
    }
    detected_count_temp
}

fn simple_scan_file(filename: &str) -> u8 {
    let mut f = File::open(filename).expect("file not found");
    let mut buf = vec![];
    f.read_to_end(&mut buf).expect("Cannot read file");
    let contents = String::from_utf8_lossy(&buf);
    //    f.read_to_string(&mut contents)
    //        println!("With text\n{}", contents);
    let add_detected_count = find_keywords(&contents);
    add_detected_count
}

fn find_keywords(content: &str) -> u8 {
    let mut detected_check: u8 = 0;
    let re = Regex::new(r"wget http://.*; chmod .*; \./.*;").unwrap();
    let cap = re.captures(content);
    if !cap.is_none() {
        detected_check = 1;
        for caps in re.captures_iter(content) {
            println!("{}", &caps[0]);
        }
    }
    detected_check
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];
    //let filename = "/home/kmasa/seccamp2020/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446";
    println!("In file/directory {}", target_path);

    let detected_count: u8;
    if check_dir(target_path) {
        detected_count = search_dir(target_path);
    } else {
        println!("Scan {}", target_path);
        detected_count = simple_scan_file(target_path);
    }
    println!("Detected count is {}", detected_count);
    //    assert!(contents.contains("wget"));
    //    matched(&contents);
}

/*
fn matched(content: &str) {
    println!("debug");
    assert!(content.contains(SIGN));
}
*/
