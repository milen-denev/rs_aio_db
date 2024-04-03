use rs_aio_db::Reflect;

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
     pub comments: String
}