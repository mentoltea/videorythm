use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba, ColorType};
use mlua::{Lua, UserData, UserDataFields, UserDataMethods, FromLua, Value};
use super::lua_pixel::LuaPixel;

#[derive(Clone)]
pub struct LuaImage {
    image: DynamicImage,
}

impl UserData for LuaImage {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("width", 
            |_, this| {
                return Ok(this.image.width());
            }
        );

        fields.add_field_method_get("height", 
            |_, this| {
                return Ok(this.image.height());
            }
        );
    }

    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("getPixel", 
            |_, this, (x, y): (u32, u32)| -> Result<LuaPixel, mlua::Error> {
                let pixel = this.image.get_pixel(x, y);
                let channels = pixel.channels();

                let luapixel = LuaPixel {
                    r: channels[0],
                    g: channels[1],
                    b: channels[2],
                    a: channels[3],
                };

                return Ok(luapixel);
            }
        );

        methods.add_method_mut("setPixel", 
            |_, this: & mut LuaImage, (x, y, pixel): (u32, u32, LuaPixel)| {
                let r = pixel.r;
                let g = pixel.g;
                let b = pixel.b;
                let a = pixel.a;

                let pixel = Rgba([r, g, b, a]);
                this.image.put_pixel(x, y, pixel);

                return Ok(());
            }
        );

        methods.add_method("copy", 
            |_, this, ()| {
                let other = this.clone();
                
                return Ok(other);
            }
        );
    }
}

impl FromLua for LuaImage {
    fn from_lua(value: Value, _: &Lua) -> Result<Self, mlua::Error> {
        match value {
            Value::UserData(ud) => {
                // Try to borrow as LuaImage
                Ok(ud.borrow::<LuaImage>()?.clone())
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaImage"),
                message: Some("Expected LuaImage userdata".into()),
            }),
        }
    }
}

pub fn execute_script_on_image(script: String, original: DynamicImage, frame_index: u32, frame_count: u32) -> DynamicImage {
    let lua = Lua::new();
    let original_image = LuaImage {image: original.clone()};
    // let edited_image = LuaImage {
    //     image: DynamicImage::new(original.width(), original.height(), original.color())
    // };
    lua.load_std_libs(mlua::StdLib::MATH).unwrap();

    let pixel_constructor = lua.create_function(
        |_, (r, g, b, a): (u8, u8, u8, u8)| {
            let pixel = LuaPixel {r, g, b, a};
            return Ok(pixel);
        }
    ).unwrap();
    lua.globals().set("Pixel", pixel_constructor).unwrap();

    let image_constructor = lua.create_function(
        |_, (width, height): (u32, u32)| {
            let image = LuaImage {
                image: DynamicImage::new(width, height, ColorType::Rgba8)
            };
            return Ok(image);
        }
    ).unwrap();
    lua.globals().set("Image", image_constructor).unwrap();

    lua.globals().set("original", original_image).unwrap();
    // lua.globals().set("edited", edited_image).unwrap();
    lua.globals().set("frame_index", frame_index).unwrap();
    lua.globals().set("frame_count", frame_count).unwrap();

    let result: LuaImage = lua.load(script).eval().unwrap();
    return result.image;
}