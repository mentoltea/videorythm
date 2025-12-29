use image::{DynamicImage, GenericImage, ImageReader, Rgba};

pub fn open_edit_save(indir: String, outdir: String, index: u32, total: u32) -> () {
    let mut origin_path = indir.clone();
    origin_path.push_str(&(index+1).to_string());
    origin_path.push_str(".png");
    println!("{}", origin_path);

    let mut edited_path = outdir.clone();
    edited_path.push_str(&(index+1).to_string());
    edited_path.push_str(".png");

    let original = ImageReader::open(origin_path).unwrap().decode().unwrap();
    let edited = apply_algorythm(&original, index, total);

    edited.save(edited_path).unwrap();
}

pub fn apply_algorythm(original: & DynamicImage, frame_index: u32, frame_count: u32) -> DynamicImage {
    let mut edited = original.clone();
    let width: u32 = edited.width();
    let height: u32 = edited.height();
    
    for x in 0..50 {
        for y in 0..50 {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut a = 255;
            
            let mut pixel: Rgba<u8> = Rgba([r,g,b,a]);
            edited.put_pixel(x, y, pixel);
        }
    }

    return edited;
}