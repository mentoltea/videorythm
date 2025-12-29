use std::{ io::{self, Write}, process };
use super::converter::Converter;

impl Converter {
    pub fn encode(& self) -> Result<(), String> {
        let mut full_video_input_path: String = self.working_directory.clone();
        full_video_input_path.push_str(&self.encoder_input_directory);
        full_video_input_path.push_str("%d.png");

        let mut full_audio_input_path: String = self.working_directory.clone();
        full_audio_input_path.push_str("audio.aac");
        
        // ffmpeg -y -i .\original_frames\%d.png -i .\audio.aac -r 240 -c:v libx264 -c:a copy -pix_fmt yuv420p out.mp4
        let mut ffmpeg_integrate_command = process::Command::new("ffmpeg");

        ffmpeg_integrate_command.arg("-y");

        ffmpeg_integrate_command.arg("-r");
        ffmpeg_integrate_command.arg(self.fps.to_string());

        ffmpeg_integrate_command.arg("-i");
        ffmpeg_integrate_command.arg(full_video_input_path);
        
        ffmpeg_integrate_command.arg("-i");
        ffmpeg_integrate_command.arg(full_audio_input_path);
        
        ffmpeg_integrate_command.arg("-r");
        ffmpeg_integrate_command.arg(self.fps.to_string());
        
        ffmpeg_integrate_command.arg("-c:v");
        ffmpeg_integrate_command.arg("libx264");

        ffmpeg_integrate_command.arg("-pix_fmt");
        ffmpeg_integrate_command.arg("yuv420p");

        ffmpeg_integrate_command.arg("-c:a");
        ffmpeg_integrate_command.arg("copy");

        ffmpeg_integrate_command.arg(self.encoder_output_path.clone());

        println!("[Encoder] Integrating video...");
        match ffmpeg_integrate_command.output() {
            Ok(out) => {
                if !out.status.success() {
                    io::stderr().write_all(&out.stderr).unwrap();
                    return Err(String::from("Audio decoder error"));
                }
            }
            Err(e) => {return Err(e.to_string());}
        }

        return Ok(());
    }
}