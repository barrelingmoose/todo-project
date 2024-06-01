mod monitor;
mod data; 
mod operations; 
mod json_helper;

use std::env; 
pub use crate::monitor::Progress;
pub use crate::data::TodoItem;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <json file>");
    }
    else{
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        operations::print_help(); 
        std::println!("=====Loading data from {}======", args.get(1).unwrap());
        operations::run(args); 
    }
}
