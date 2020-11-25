use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

static mut SCAN_COUNT: u64 = 0;
static mut CHMOD_COUNT: u64 = 0;
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
                SCAN_COUNT = SCAN_COUNT + 1;
                println!("Now Scaning {} files", SCAN_COUNT);
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
    let re_wget =
        Regex::new(r"(wget|curl)\s+.*(?P<wget_file>https?://[\w/:%#\$&\?\(\)~\.=\+\-]+)\s*((;|\&\&)\s*.*(;|\&\&)*\s*chmod|\n)").unwrap();
    if let Some(_caps_wget) = re_wget.captures(content) {
        let mut loop_count = 0;
        for cap_wget in re_wget.captures_iter(content) {
            // println!("{}", &cap_wget["wget_file"]);
            let wget_str = &cap_wget["wget_file"];
            println!("WGET(debug)::{}", wget_str);
            let re_chmod =
            Regex::new(r"chmod\s+[+\w=]+\s*(?P<chmod_file>[\w/:%#\$&\?\(\)~=\+\-*]+)[\.\w-]*\s*((;|\&\&)\s*(\./|sh\s+)|\n)").unwrap();

            if let Some(cap_chmod) = re_chmod.captures(content) {
                let chmod_str = &cap_chmod["chmod_file"];
                println!("CHMOD(debug)::{}", chmod_str);
                if wget_str.contains(chmod_str) || chmod_str == "*" {
                    if loop_count == 0 {
                        unsafe {
                            CHMOD_COUNT = CHMOD_COUNT + 1;
                        }
                    }
                    println!("chmod_detected(debug)!!!!!!!!");
                }
                let re_exec = Regex::new(
                    r"([^\.]\./|sh\s+)(?P<exec_file>[\w/:%#\$&\?\(\)~\.=\+\-*]+)\s*(;|\&\&|\n)",
                )
                .unwrap();
                if let Some(cap_exec) = re_exec.captures(content) {
                    let exec_str = &cap_exec["exec_file"];
                    println!("EXEC(debug)::{}", exec_str);
                    if wget_str.contains(exec_str) || exec_str == "*" {
                        detected_check = 1;
                        println!("exec_detected(debug)!!!!!!!!!!!\n");
                        break;
                    }
                }
            }
            loop_count = loop_count + 1;
        }
    }
    detected_check
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];
    let init_detected_count = 0;
    let scan_count: u64;
    let chmod_count: u64;
    //let filename = "~/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446";
    println!("In file/directory {}", target_path);

    let detected_count: u64;
    if check_dir(target_path) {
        detected_count = search_dir(target_path, init_detected_count);
        unsafe {
            scan_count = SCAN_COUNT;
        }
        println!("\n-----> All scan {} files", scan_count);
    } else {
        println!("Scan {}", target_path);
        detected_count = simple_scan_file(target_path);
        let allscanfiles_count = 1;
        println!("\n-----> Scan {} file", allscanfiles_count);
    }
    unsafe {
        chmod_count = CHMOD_COUNT;
    }
    println!("\n-----> Chmod Count(debug) {} files", chmod_count);
    println!("-----> Detected {} files", detected_count);
    //    assert!(contents.contains("wget"));
    //    matched(&contents);
}
