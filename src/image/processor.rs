use image::{DynamicImage, ImageReader};
use crate::lua_binding::lua_image;

pub fn open_edit_save(indir: String, outdir: String, script:String, index: u32, total: u32) -> () {
    let mut origin_path = indir.clone();
    origin_path.push_str(&(index+1).to_string());
    origin_path.push_str(".png");
    println!("{}", origin_path);

    let mut edited_path = outdir.clone();
    edited_path.push_str(&(index+1).to_string());
    edited_path.push_str(".png");

    let original = ImageReader::open(origin_path).unwrap().decode().unwrap();
    let edited = apply_algorythm(&original, script, index, total);

    edited.save(edited_path).unwrap();
}

pub fn apply_algorythm(original: & DynamicImage, script:String, frame_index: u32, frame_count: u32) -> DynamicImage {
    let edited = lua_image::execute_script_on_image(
        script, 
        original.clone(), 
        frame_index, 
        frame_count
    );

    return edited;
}