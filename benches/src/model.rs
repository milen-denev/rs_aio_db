use std::collections::HashMap;

use rs_aio_db::Reflect;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Reflect)]
pub struct Person {
     pub id: u32,
     pub first_name: String,
     pub last_name: String,
     pub age: u32,
     pub height: f32,
     pub married: bool,
     pub address: String,
     pub date_of_birth: u32,
     pub comments: String,
     pub some_blob: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct AnotherStruct {
    pub data_1: i32,
    pub data_2: f64,
    pub data_3: HashMap<String, String>
}