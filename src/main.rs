use image_duplicate_checker::Arguments;
use std::env;
use std::process;
fn main() {
    let args = Arguments::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    image_duplicate_checker::run(&args);
}
