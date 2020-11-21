use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

static mut Scan_Count: u64 = 0;
// "~/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446")
/*
fn scanfile_count(add_scanfile_num: u64, caller_check: u8) -> u64 {
    let mut scanfile_num: u64 = add_scanfile_num;
    if caller_check == 1 {
        scanfile_num = scanfile_num + 1;
    }
    scanfile_num
}
*/
fn check_dir(target_path: &str) -> bool {
    let check = metadata(target_path).unwrap();
    check.is_dir()
}

fn search_dir(target_path: &str, mut detected_count: u64) -> u64 {
    let paths = fs::read_dir(target_path).unwrap();
    let mut return_detected_count: u64 = 0;
    for path in paths {
        let path_str: &str = &(path.unwrap().path().display()).to_string();
        if check_dir(path_str) {
            detected_count = search_dir(path_str, detected_count);
        } else {
            println!("Scan {}", path_str);
            return_detected_count = simple_scan_file(path_str);
            unsafe {
                Scan_Count = Scan_Count + 1;
                println!("Now Scaning {} files", Scan_Count);
            }
        }
        detected_count = detected_count + return_detected_count;
    }
    //    println!("\n-----> Detected {} files", detected_count);
    detected_count
}

fn simple_scan_file(filepath: &str) -> u64 {
    let mut fr = File::open(filepath).expect("file not found");
    let mut buf = vec![];
    fr.read_to_end(&mut buf).expect("Cannot read file");
    let contents = String::from_utf8_lossy(&buf);
    //    f.read_to_string(&mut contents)
    //        println!("With text\n{}", contents);
    let add_detected_count: u64 = find_keywords(&contents);
    if add_detected_count == 0 {
        println!("Undetected File: {}", filepath);
    }
    add_detected_count
}

fn find_keywords(content: &str) -> u64 {
    let mut detected_check: u64 = 0;
    //    let re = Regex::new(r"[wget|curl]\s+(?P<wget_file>http.*)\s*[;|\&\&|\n][\s\S]*chmod\s+(?P<chmod_file>.*)\s*[;|\&\&|\n][\s\S]*[\./|sh[\s\S]*](?P<exec_file>.*)\s*[;|\&\&|\n]")
    //            .unwrap();
    //    let re = Regex::new(r"[wget|curl]\s+(?P<wget_file>http(s)?:.*)\s*[;|\&\&|\n]").unwrap();
    let re_wget =
        Regex::new(r"[wget|curl]\s+[\s\-\w=,]*(?P<wget_file>http://.*)\s*[;|\&\&|\n]").unwrap();
    let cap_wget = re_wget.captures(content);
    if !cap_wget.is_none() {
        let re_chmod = Regex::new(r"chmod\s+[\s\-\w=,]*(?P<chmod_file>.*)\s*[;|\&\&|\n]").unwrap();
        let cap_chmod = re_chmod.captures(content);
        //        println!("passsssss");
        if !cap_chmod.is_none() {
            let re_exec = Regex::new(r"[\./|sh\s+](?P<exec_file>.*)\s*[;|\&\&|\n]").unwrap();
            let cap_exec = re_exec.captures(content);
            if !cap_exec.is_none() {
                detected_check = 1;
            }
        }
    }
    detected_check
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];
    let init_detected_count = 0;
    let scan_count: u64;
    //let filename = "~/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446";
    println!("In file/directory {}", target_path);

    let detected_count: u64;
    if check_dir(target_path) {
        detected_count = search_dir(target_path, init_detected_count);
    } else {
        println!("Scan {}", target_path);
        detected_count = simple_scan_file(target_path);
        let allscanfiles_count = 1;
        println!("\n-----> Scan {} file", allscanfiles_count);
    }
    unsafe {
        scan_count = Scan_Count;
    }
    println!("\n-----> All scan {} files", scan_count);
    println!("-----> Detected {} files", detected_count);
    //    assert!(contents.contains("wget"));
    //    matched(&contents);
}
