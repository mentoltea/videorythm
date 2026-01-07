use crate::video::{converter};
use crate::threader::state::ControllerState;
use std::thread;
use std::num::NonZero;
use std::fs;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub work_dir: String,
    
    pub original_dir: String,
    pub edited_dir: String,
    
    pub input_file: String,
    pub output_file: String,
    
    pub script_file: String,
    
    pub use_cached: bool,
    pub fps: u16,

    pub threads: u16,
}


pub fn process_video(
    args: Arguments
) {
    let mut converter = converter::Converter::new(
        args.input_file.clone(),
        args.work_dir.clone(), 
        
        args.original_dir.clone(), 
        args.use_cached,

        args.edited_dir.clone(),
        args.output_file.clone(),

        args.fps,
    );

    match converter.decode() {
        Ok(_) => { println!("[Decoder] {} frames", converter.frames); }
        Err(e) => {
            println!("Error while decoding has happened:");
            println!("\t{}", e);
            return;
        }
    }

    let mut full_outdir = args.work_dir.clone();
    full_outdir.push_str(&args.edited_dir);
    
    if !fs::exists(full_outdir.clone()).unwrap() {
        fs::create_dir_all(full_outdir.clone()).unwrap();
    }

    let mut full_indir = args.work_dir.clone();
    full_indir.push_str(&args.original_dir);

    let script: String = fs::read_to_string(args.script_file).unwrap();

    let controller = ControllerState::new(
        full_indir.clone(), 
        full_outdir.clone(), 
        script.clone(), 
        converter.frames
    );

    let threads_available = thread::available_parallelism().unwrap();
    let user_threads = NonZero::new(args.threads as usize);

    match user_threads {
        Some(x) => controller.start(x),
        None => controller.start(threads_available),
    };
    
    controller.join();


    match converter.encode() {
        Ok(_) => { println!("[Encoder] {} frames", converter.frames); }
        Err(e) => {
            println!("Error while encoding has happened:");
            println!("\t{}", e);
            return;
        }
    }
}