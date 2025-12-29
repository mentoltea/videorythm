mod video;
mod image;
mod threader;
mod logic;

fn main() {
    let work_dir = String::from("./example/");

    let indir = String::from("original_frames/");
    let outdir = String::from("edited_frames/");

    let infile = String::from("./example.mp4");
    let outfile = String::from("out.mp4");

    let use_cached = true;
    let fps = 30;

    logic::process_video(infile, outfile, work_dir, indir, outdir, fps, use_cached);

    println!("Finished");
}
