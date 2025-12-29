use video::decoder;

mod video;

fn main() {
    let decoder = decoder::Decoder::new(
        String::from("./example.mp4"),
        String::from("./example/"), 
        String::from("original_frames/"), 
        true,
        30,
    );

    match decoder.decode() {
        Ok(x) => { println!("[Decoder] {} frames", x); }
        Err(e) => {
            println!("Error while decoding has happened:");
            println!("\t{}", e);
        }
    }

    println!("Finished");
}
