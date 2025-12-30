use mlua::{Lua, UserData, UserDataFields, FromLua, Value};

#[derive(Clone)]
pub struct LuaPixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl UserData for LuaPixel {
    fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
        
        fields.add_field_method_get("r", 
        |_, this| {
            return Ok(this.r);
        }
        );
        fields.add_field_method_set("r", 
            |_, this, new: u8| {
                this.r = new;
                return Ok(());
            }
        );

        fields.add_field_method_get("g", 
        |_, this| {
            return Ok(this.g);
        }
        );
        fields.add_field_method_set("g", 
            |_, this, new: u8| {
                this.g = new;
                return Ok(());
            }
        );

        fields.add_field_method_get("b", 
        |_, this| {
            return Ok(this.b);
        }
        );
        fields.add_field_method_set("b", 
            |_, this, new: u8| {
                this.b = new;
                return Ok(());
            }
        );

        fields.add_field_method_get("a", 
        |_, this| {
            return Ok(this.a);
        }
        );
        fields.add_field_method_set("a", 
            |_, this, new: u8| {
                this.a = new;
                return Ok(());
            }
        );

    }
}

impl FromLua for LuaPixel {
    fn from_lua(value: Value, _: &Lua) -> Result<Self, mlua::Error> {
        match value {
            Value::UserData(ud) => {
                Ok(ud.borrow::<LuaPixel>()?.clone())
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: String::from("LuaPixel"),
                message: Some("Expected LuaPixel userdata".into()),
            }),
        }
    }
}