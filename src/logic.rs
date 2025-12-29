use crate::video::{converter};
use crate::threader::state::ControllerState;
use std::thread;
use std::fs;


pub fn process_video(
    infile: String,
    outfile: String,
    work_dir: String,
    indir: String,
    outdir: String,
    fps: u16,
    use_cached: bool
) {
    let mut converter = converter::Converter::new(
        infile.clone(),
        work_dir.clone(), 
        
        indir.clone(), 
        use_cached,

        outdir.clone(),
        outfile.clone(),

        fps,
    );

    match converter.decode() {
        Ok(_) => { println!("[Decoder] {} frames", converter.frames); }
        Err(e) => {
            println!("Error while decoding has happened:");
            println!("\t{}", e);
            return;
        }
    }

    let mut full_outdir = work_dir.clone();
    full_outdir.push_str(&outdir);
    
    if !fs::exists(full_outdir.clone()).unwrap() {
        fs::create_dir_all(full_outdir.clone()).unwrap();
    }

    let mut full_indir = work_dir.clone();
    full_indir.push_str(&indir);


    let controller = ControllerState::new(full_indir.clone(), full_outdir.clone(), converter.frames);

    let threads_available = thread::available_parallelism().unwrap();

    controller.start(threads_available);
    
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