use crate::config::ModelConfigSettingType;
use crate::plugin::{ModelPrompt, ModelPromptBinaryPayload};
use anyhow::{format_err, Error};
use rquickjs::{Ctx, TypedArray, Value};

pub trait JSTypeExt {
    fn to_value(self, ctx: Ctx) -> Result<Value, Error>;

    fn from_value(v: Value) -> Result<Self, Error>
    where
        Self: Sized;
}

impl JSTypeExt for ModelConfigSettingType {
    fn to_value(self, ctx: Ctx) -> Result<Value, anyhow::Error> {
        Ok(match self {
            ModelConfigSettingType::Float(f) => Value::new_float(ctx.clone(), f),
            ModelConfigSettingType::Integer(i) => Value::new_int(ctx.clone(), i),
            ModelConfigSettingType::Bool(b) => Value::new_bool(ctx.clone(), b),
            ModelConfigSettingType::String(s) => {
                rquickjs::String::from_str(ctx.clone(), s.as_str())?
                    .as_value()
                    .clone()
            }
        })
    }

    fn from_value(v: Value) -> Result<Self, Error> {
        if let Some(number) = v.as_int() {
            Ok(ModelConfigSettingType::Integer(number))
        } else if let Some(float) = v.as_float() {
            Ok(ModelConfigSettingType::Float(float))
        } else if let Some(string) = v.as_string() {
            Ok(ModelConfigSettingType::String(string.to_string()?))
        } else if let Some(bool) = v.as_bool() {
            Ok(ModelConfigSettingType::Bool(bool))
        } else {
            Err(format_err!(
                "Type '{}' is not supported for plugin settings",
                v.type_name()
            ))
        }
    }
}

impl JSTypeExt for ModelPrompt {
    fn to_value(self, ctx: Ctx) -> Result<Value, Error> {
        let object = rquickjs::Object::new(ctx.clone())?;

        Ok(match self {
            ModelPrompt::Text(t) => {
                object.set("type", "text")?;
                object.set("value", t)?;
                object.as_value().clone()
            }
            ModelPrompt::Image(p) => {
                let value = TypedArray::new(ctx.clone(), p.value)?;
                object.set("type", "image")?;
                object.set("value", value)?;
                object.set("format", p.format)?;
                object.as_value().clone()
            }
            ModelPrompt::Document(p) => {
                let value = TypedArray::new(ctx.clone(), p.value)?;
                object.set("type", "document")?;
                object.set("value", value)?;
                object.set("format", p.format)?;
                object.as_value().clone()
            }
        })
    }

    fn from_value(v: Value) -> Result<Self, Error> {
        match v.as_object() {
            None => Err(format_err!(
                "model response should be an object, {} returned.",
                v.type_name()
            )),
            Some(object) => {
                let prompt_type: String = object.get("type")?;

                match prompt_type.as_str() {
                    "text" => {
                        let text = object.get("value")?;

                        Ok(ModelPrompt::Text(text))
                    }
                    "image" => {
                        let value: rquickjs::TypedArray<u8> = object.get("value")?;
                        let format = object.get("format")?;

                        Ok(ModelPrompt::Image(ModelPromptBinaryPayload {
                            value: Vec::from(value.as_ref() as &[u8]),
                            format,
                        }))
                    }
                    "document" => {
                        let value: rquickjs::TypedArray<u8> = object.get("value")?;
                        let format = object.get("format")?;

                        Ok(ModelPrompt::Image(ModelPromptBinaryPayload {
                            value: Vec::from(value.as_ref() as &[u8]),
                            format,
                        }))
                    }
                    _ => Err(format_err!(
                        "Type '{}' is not supported for plugin prompt",
                        prompt_type
                    )),
                }
            }
        }
    }
}
