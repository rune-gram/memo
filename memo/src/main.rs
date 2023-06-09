mod vms;
mod parser;
mod engine;
use std::io::Read;


fn main() {
    // get args
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter();
    args.next();
    let name = args.next().unwrap();
    let ext = name.split('.').last().unwrap();
    if ext != "memo" {panic!("File extension must be .memo");}
    // load file bytecode
    let mut file = std::fs::File::open(name).unwrap();
    // let mut contents = Vec::new();
    // file.read_to_end(&mut contents).unwrap();
    // read file as text
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    parser::scan::Scans::new(contents).start();
}