# VideoRythm
(composed of 'video' + 'algorythm')  
Is a program for editing video frame-by-frame with lua scripts

## Logic

The process is divided into several steps:
- Decomposing original video to png's (with ffmpeg)
- Editing each image independently (with provided lua script)
- Integrating frames back into one video, restoring the sound (with ffmpeg)

## Usage

```bash
cargo run -- -i [inputfile] -o [outputfile] -s [scriptfile] [...other parameters]
```

## Parameters

- `--script` or `-s`  

Specifies the lua scipt filepath.
The script **must return an image** (later in [Lua lang](#lua-specifications))  
Is obligatory.


- `--input` or `-i`  

Specifies the input video filepath.
Is obligatory.


- `--output` or `-o`  

Specifies the output video filepath.  
By default is `./[input_filename]_out.[input_extenstion]`


- `--threads` or `-t`

Specifies the maximum number of threads to use.
By default is number of threads available on a platform.
_Note: does not change number of threads for ffmpeg_ 


- `--fps`

Sets framerate of decoding & encoding.
By default is `30`.


- `--use_cached`

If the parameter is `true` and there is cached data that can be used (has the same parameters), uses it instead of recomputing.
Affects only decoding step.
By default is `true` .


- `--working`

Specifies programm working directory.
By default is `./[input_filename]_pwd/`


- `--original_dir`

Specifies directory of decoder's output inside working directory.
By default is `original_frames/`.


- `--edited_dir`

Specifies directory of encoder's output inside working directory.
By default is `edited_frames/`.



## Lua specifications

### Lua Pixel
Is a class representing RGBA8 pixel with corresponding fields `.r`, `.g`, `.b` and `.a` available from Lua (both getters and setters) .
Also there is a function to create new pixel objects ( described in pseudocode, as Lua OOP is not as intuitive ) :
``` rust
fn Pixel( r: u32, g: u32, b: u32, a: u32) -> LuaPixel
```

### Lua Image
Is a class wrapper for `DynamicImage` from rust's `image` crate.
Also there is a function to create new image objects.
Below is described _API available from Lua_ ( in pseudocode, as Lua OOP is not as intuitive ) .
``` rust
class LuaImage {
    fn width( &self ) -> u32
    fn height( &self ) -> u32
    fn getPixel( &self, x: u32, y: u32 ) -> LuaPixel
    fn setPixel( &mut self, x: u32, y: u32, pixel: LuaPixel ) -> ()
    fn copy( &self ) -> LuaImage
}
fn Image( width: u32, height: u32 ) -> LuaImage
```

### Global variables

- `original`

`LuaImage` object of input image.


- `frame_index`

Index of the frame currently processed.


- `frame_count`

Total number of input frames.


### Return value
It is expected for script to return `LuaImage` object of edited image at the end (it can be both altered original image or a new one).

## Dependencies
[ffmpeg](https://ffmpeg.org/)
[mlua](https://docs.rs/mlua/latest/mlua/)
[image](https://docs.rs/image/latest/image/)