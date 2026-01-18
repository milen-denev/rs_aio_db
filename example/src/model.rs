use std::collections::HashMap;

use rs_aio_db::{Deserialize, Reflect, Serialize};

#[derive(Default, Clone, Debug, Reflect)]
pub struct Person {
     pub name: String,
     pub age: i32,
     pub height: i32,
     pub married: bool,
     pub some_blob: Vec<u8>
}

#[derive(Serialize, Deserialize)]
pub struct AnotherStruct {
    pub data_1: i32,
    pub data_2: f64,
    pub data_3: HashMap<String, String>
}