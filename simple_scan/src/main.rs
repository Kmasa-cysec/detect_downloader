use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    for result in BufReader::new(File::open("/home/kmasa/seccamp2020/dataset/rm/03ec5e176ea404f1193608a4298a5ebdaa2e275461836b6762d25cf19b252446")?).lines() {
        let l = result?;
        println!("{}", l);
    }
    Ok(())
}
