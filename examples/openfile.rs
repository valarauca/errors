

#[macro_use]
extern crate errors;
use std::fs::File;

fn main() {

    let filename = exitf!(get_args(), "{} {} {}", "a", "few", "messages");
    let file = exitf!(File::open(&filename), "failed to open file {}", &filename);
}

fn get_args() -> Result<String,String> {
    let args = ::std::env::args().collect::<Vec<_>>();
    if args.len() <= 1 {
        Err("not enough args".to_string())
    } else {
        Ok(args[1].clone())
    }
}
