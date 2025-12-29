use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use mlua::{Lua, Table, UserData, UserDataFields, UserDataMethods, FromLua, Value};

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
            |lua, this, (x, y): (u32, u32)| {
                let pixel = this.image.get_pixel(x, y);
                let channels = pixel.channels();

                let table = lua.create_table()?;
                table.set("r", channels[0]).unwrap();
                table.set("g", channels[1]).unwrap();
                table.set("b", channels[2]).unwrap();
                table.set("a", channels[3]).unwrap();

                return Ok(table);
            }
        );

        methods.add_method_mut("setPixel", 
            |_, this: & mut LuaImage, (x, y, color): (u32, u32, Table)| {
                let r = color.get("r").unwrap_or(0);
                let g = color.get("g").unwrap_or(0);
                let b = color.get("b").unwrap_or(0);
                let a = color.get("a").unwrap_or(255);

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

    lua.globals().set("original", original_image).unwrap();
    // lua.globals().set("edited", edited_image).unwrap();
    lua.globals().set("frame_index", frame_index).unwrap();
    lua.globals().set("frame_count", frame_count).unwrap();

    let result: LuaImage = lua.load(script).eval().unwrap();
    return result.image;
}