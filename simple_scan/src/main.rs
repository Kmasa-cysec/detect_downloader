//use std::env;
use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

// "/home/kmasa/seccamp2020/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446")

//const SIGN: &str = "wget http://";
//const SIGN = Regex::new(r"wget http://+;").unwrap();

fn main() {
//    let args: Vec<String> = env::args().collect();
//    let filename = &args[1];
    let filename = "/home/kmasa/seccamp2020/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446";

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text\n{}", contents);

//    assert!(contents.contains("wget"));
//    matched(&contents);
    find_keywords(&contents);
}

/*
fn matched(content: &str) {
    println!("debug");
    assert!(content.contains(SIGN));
}
*/
fn find_keywords(content: &str) {
    let re = Regex::new(r"wget http://.*; chmod .*;").unwrap();
    for caps in re.captures_iter(content) {
        println!("{}", &caps[0]);
    }
}


