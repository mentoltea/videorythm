use std::{ fs, io::{self, Write}, process };
use super::converter::Converter;

impl Converter {
    pub fn decode(& mut self) -> Result<(), String> {
        if !fs::exists(self.working_directory.clone()).unwrap() {
            match fs::create_dir_all(self.working_directory.clone()) {
                Ok(_) => {}
                Err(e) => {return Err(e.to_string())}
            }
        }
        

        let mut full_output_directory: String = String::new();
        full_output_directory.push_str(&self.working_directory);
        full_output_directory.push_str(&self.decoder_output_directory);

        if !fs::exists(full_output_directory.clone()).unwrap() {
            match fs::create_dir(full_output_directory.clone()) {
                Ok(_) => {}
                Err(e) => {return Err(e.to_string())}
            }
        }
        
        let mut finished_flag_path: String = self.working_directory.clone();
        finished_flag_path.push_str("decode.finished.flag");
        
        let mut can_use_cache = false;
        let mut used_cache = false;
        let mut cache_frames: u32 = 0;
        
        if fs::exists(finished_flag_path.clone()).unwrap() {
            let mut cache_fps: u16 = 0;
            let mut decoder_output_dir = String::new();
            let finished_flag_content = fs::read_to_string(finished_flag_path.clone()).unwrap();
            for line in finished_flag_content.lines() {
                if line.starts_with("fps=") {
                    let fps_str = &line[4..];
                    cache_fps = fps_str.parse::<u16>().unwrap_or(0);
                } else 
                if line.starts_with("frames=") {
                    let frame_str = &line[7..];
                    cache_frames = frame_str.parse::<u32>().unwrap_or(0);
                }
                if line.starts_with("dir=") {
                    let dir_str = &line[4..];
                    decoder_output_dir = dir_str.to_owned();
                }
            }
            can_use_cache = (self.fps == cache_fps) 
                && cache_frames>0 
                && (decoder_output_dir == self.decoder_output_directory);
            if !can_use_cache {
                println!("[Decoder] Cached result is not suitable, recomputation needed");
            }
        }

        if !can_use_cache || !self.decoder_use_cached {         
            let mut ffmpeg_decompose_video_command = process::Command::new("ffmpeg");
            
            ffmpeg_decompose_video_command.arg("-y");
            
            ffmpeg_decompose_video_command.arg("-i");
            ffmpeg_decompose_video_command.arg(self.filepath.clone());
            
            ffmpeg_decompose_video_command.arg("-r");
            ffmpeg_decompose_video_command.arg(self.fps.to_string());
            
            let mut ffmpeg_output_path = full_output_directory.clone();
            ffmpeg_output_path.push_str("%d.png");
            
            ffmpeg_decompose_video_command.arg(ffmpeg_output_path);
            
            println!("[Decoder] Decomposing video at {} fps...", self.fps);
            match ffmpeg_decompose_video_command.output() {
                Ok(out) => {
                    if !out.status.success() {
                        io::stderr().write_all(&out.stderr).unwrap();
                        return Err(String::from("Video decoder error"));
                    }
                }
                Err(e) => {return Err(e.to_string());}
            }
            
            
            println!("[Decoder] Extracting audio...");
            let mut ffmpeg_extract_audio_command = process::Command::new("ffmpeg");
            
            ffmpeg_extract_audio_command.arg("-y");

            ffmpeg_extract_audio_command.arg("-i");
            ffmpeg_extract_audio_command.arg(self.filepath.clone());

            ffmpeg_extract_audio_command.arg("-vn"); // no video
            ffmpeg_extract_audio_command.args(["-acodec", "copy"]); // copy audio without reencoding
            

            let mut ffmpeg_output_path = self.working_directory.clone();
            ffmpeg_output_path.push_str("audio.aac");

            ffmpeg_extract_audio_command.arg(ffmpeg_output_path); // copy audio without reencoding

            match ffmpeg_extract_audio_command.output() {
                Ok(out) => {
                    if !out.status.success() {
                        io::stderr().write_all(&out.stderr).unwrap();
                        return Err(String::from("Audio decoder error"));
                    }
                }
                Err(e) => {return Err(e.to_string());}
            }
        } else {
            println!("[Decoder] Using cached results");
            used_cache = true;
        }

        let mut frames: u32 = cache_frames;
        if !used_cache {
            // Count the number of frames
            let dir = fs::read_dir(full_output_directory).unwrap();
            let mut counter: u32 = 0;
            for entry in dir {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file() {
                    let filename = entry.file_name();
                    if filename.to_str().unwrap().ends_with(".png") {
                        counter += 1;
                    }
                }

            }
            frames = counter;
        }
        self.frames = frames;

        let mut finished_flag_file = fs::File::create(finished_flag_path.clone()).unwrap();

        write!(finished_flag_file, "fps={}\n", self.fps).unwrap();
        write!(finished_flag_file, "frames={}\n", self.frames).unwrap();
        write!(finished_flag_file, "dir={}\n", self.decoder_output_directory).unwrap();

        return Ok(());
    }
}