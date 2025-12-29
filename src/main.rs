use video::{converter};

mod video;

fn main() {
    let mut converter = converter::Converter::new(
        String::from("./example.mp4"),
        String::from("./example/"), 
        
        String::from("original_frames/"), 
        true,

        String::from("original_frames/"),
        String::from("out.mp4"),

        30,
    );

    match converter.decode() {
        Ok(_) => { println!("[Decoder] {} frames", converter.frames); }
        Err(e) => {
            println!("Error while decoding has happened:");
            println!("\t{}", e);
            return;
        }
    }

    match converter.encode() {
        Ok(_) => { println!("[Encoder] {} frames", converter.frames); }
        Err(e) => {
            println!("Error while encoding has happened:");
            println!("\t{}", e);
            return;
        }
    }

    println!("Finished");
}
