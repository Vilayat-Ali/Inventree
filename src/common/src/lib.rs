use std::fmt::Debug;

use serde::*;

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Response<T> {
    status: u32,
    success: bool,
    message: String,
    data: Option<T>,
}

impl<T> Response<T>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    // new instance
    pub fn new(
        status: u32,
        success: bool,
        message: String,
        data: Option<T>,
    ) -> Self {
        Self {
            status,
            success,
            message,
            data,
        }
    }

    // convert to json
    pub fn to_json(&self) -> serde_json::Result<String> {
        let json_string: String = serde_json::to_string(&self)?;
        Ok(json_string)
    }
}

// util functions for Frontend uses

// serialize to json
pub fn parse_into_json<T>(payload: &T) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    let json_string: String = serde_json::to_string(payload)?;
    Ok(json_string)
}

// serialize back to native rust data types
pub fn parse_into_native<T>(json_string: &str) -> Result<T, serde_json::Error>
where
    T: Serialize + for<'a> Deserialize<'a>,
{
    let native_data_structure = serde_json::from_str(json_string)?;
    Ok(native_data_structure)
}

// example
// let r: Response<String> = Response::new(200, String::from("I love Rust!"), None);
// println!("{:?}", r.to_json().unwrap());
// let j = parse_into_json(&r).unwrap();
// println!("{}", j);
// let a: Response<String> = parse_into_native(&j).unwrap();
// println!("{:?}", a);
