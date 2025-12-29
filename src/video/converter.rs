use std::fs;

pub struct Converter {
    pub filepath: String,
    pub working_directory: String,

    pub decoder_output_directory: String,
    pub decoder_use_cached: bool,
    
    pub encoder_input_directory: String,
    pub encoder_output_path: String,

    pub fps: u16,
    pub frames: u32,
}

impl Converter {
    pub fn new(
        filepath: String, 
        working_directory: String, 

        output_directory: String, 
        use_cached: bool, 

        encoder_input_directory: String,
        encoder_output_path: String, 

        fps: u16,
    ) -> Self {
        let new = Converter {
            filepath: filepath,
            working_directory: working_directory,
            decoder_output_directory: output_directory,
            decoder_use_cached: use_cached,
            encoder_input_directory: encoder_input_directory,
            encoder_output_path: encoder_output_path,
            fps: fps,
            frames: 0,
        };
        return new;
    }

    pub fn clear(& self) -> Result<(), String> {
        fs::remove_dir_all(self.working_directory.clone()).unwrap();
        return Err(String::from("Not implemented"));
    }
}
