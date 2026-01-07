mod video;
mod image;
mod threader;
mod logic;
mod cli;
mod lua_binding;

use std::{env};

fn main() {
    let raw_args: Vec<String> = env::args().collect();

    let args = cli::parse_command_line(&raw_args[1..]); 
    if let Err(x) = args {
        println!("Argument parsing error: {x}");
        return;
    }

    logic::process_video(args.unwrap());
    
    println!("Finished");
}
