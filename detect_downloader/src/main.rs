use std::env;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::panic;
extern crate regex;
use regex::Regex;

static mut SCAN_COUNT: i32 = 0;
static mut CHMOD_COUNT: i32 = 0;
fn check_dir(target_path: &str) -> bool {
    let check = metadata(target_path).unwrap();
    check.is_dir()
}

fn search_dir(target_path: &str, mut detected_count: i32) -> i32 {
    let paths = fs::read_dir(target_path).unwrap();
    let mut return_detected_count: i32 = 0;
    for path in paths {
        let path_str: &str = &(path.unwrap().path().display()).to_string();
        if check_dir(path_str) {
            detected_count = search_dir(path_str, detected_count);
        } else {
            unsafe {
                SCAN_COUNT = SCAN_COUNT + 1;
                println!("\n<Now Scaning {} files>", SCAN_COUNT);
            }
            println!("Scan {}", path_str);
            return_detected_count = simple_scan_file(path_str);
        }
        detected_count = detected_count + return_detected_count;
    }
    detected_count
}

fn simple_scan_file(filepath: &str) -> i32 {
    let mut fr = File::open(filepath).expect("file not found");
    let mut buf = vec![];
    fr.read_to_end(&mut buf).expect("Cannot read file");
    let contents = String::from_utf8_lossy(&buf);
    let add_detected_count: i32 = find_keywords(&contents);
    if add_detected_count == 0 {
        println!("Undetected File: {}", filepath);
    }
    add_detected_count
}

fn find_keywords(content: &str) -> i32 {
    let mut detected_check = 0;
    let mut dc_ref = panic::AssertUnwindSafe(&mut detected_check);
    let mut check_chmod = 0;
    let _p = panic::catch_unwind(move || {
        let match_wget = r"(wget|curl)\s+(\$[\w\{\}]*)*\s*[+\w=\-_]*\s*(?P<wget_file>(https?://)?[\w/:%#&\$\?\{\}\(\)~\.=_\+\-]+(\s*\-O\s*[\.\w\$\-_/]*)*)\s*(.*\s*(;|\&\&)\s*.*(;|\&\&)*\s*chmod|[.\s]*\|\||\n)";
        let re_wget = Regex::new(match_wget.trim()).unwrap();
        if let Some(_caps_wget) = re_wget.captures(content) {
            'outside: for cap_wget in re_wget.captures_iter(content) {
                let mut wget_str_tmp = &cap_wget["wget_file"];
                let wget_str_parse: Vec<&str>;
                if wget_str_tmp.contains(char::is_whitespace) {
                    wget_str_parse = wget_str_tmp.split_whitespace().collect();
                } else {
                    wget_str_parse = wget_str_tmp.split("/").collect();
                }
                wget_str_tmp = wget_str_parse[wget_str_parse.len() - 1];
                let wget_str;
                println!("WGET(debug)::{}", wget_str_tmp);
                if wget_str_tmp.starts_with(r"$") {
                    wget_str = wget_str_tmp.replace(r"$", r"\$");
                } else {
                    wget_str = wget_str_tmp.to_string();
                }
                let match_chmod = format!(
                    //r"chmod\s+([\w=\-]+\s*)*\s*.*({}|\*)\s*(;|\&\&|\|\||\n)",
                    //r"chmod\s+[\s\S]*\s*({}[\.\w\$\-_]*|\*)\s*(;|\&\&|\|\||\n)",
                    r"chmod\s+[\w=\-\.\+]*\s*([\.\w\$\-_/]*{}[\.\w\$\-_/]*|\*)\s*(;|\&\&|\|\||\n)",
                    wget_str
                );
                let re_chmod = Regex::new(match_chmod.trim()).unwrap();
                if let Some(caps_chmod) = re_chmod.captures(content) {
                    println!(
                        "CHMOD(debug)::{}",
                        caps_chmod.get(0).map_or("", |m| m.as_str())
                    );
                    if check_chmod == 0 {
                        unsafe {
                            CHMOD_COUNT = CHMOD_COUNT + 1;
                            check_chmod = check_chmod + 1;
                        }
                    }
                    let match_exec = format!(
                        //r"(\&\&|;|\n|\|\|)\s*(\./|[^\.]/[^\n;\&\s]*|sh\s+)\s*({}|\*^\+)\s*[^\n;\*]*(>+|;|\n)",
                        r"(\&\&|;|\n|\|\|)\s*(\.*/|sh\s+|perl\s+)\s*[\w\.\$\-_=/\&]*({}|\*^\+)\s*[^\n;\*]*(>+|;|\n)",
                        wget_str
                    );
                    let re_exec = Regex::new(match_exec.trim()).unwrap();
                    if let Some(cap_exec) = re_exec.captures(content) {
                        println!(
                            "EXEC(debug)::{}",
                            cap_exec.get(0).map_or("", |m| m.as_str())
                        );
                        **dc_ref = 1;
                        println!("exec_detected(debug)!!!!!!!!!!!\n");
                        break 'outside;
                    }
                }
            }
        }
    });
    detected_check
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path = &args[1];
    let init_detected_count = 0;
    let mut scan_count: i32 = 0;
    let chmod_count: i32;
    println!("In file/directory {}", target_path);
    let detected_count: i32;
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
    println!(
        "\n-----> TP Rate: {:.3}% ",
        (detected_count as f32 / scan_count as f32) * 100.0
    );
}
