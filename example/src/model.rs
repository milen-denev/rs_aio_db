use rs_aio_db::Reflect;

#[derive(Default, Clone, Debug, Reflect)]
pub struct Person {
     pub name: String,
     pub age: i32,
     pub height: i32,
     pub married: bool,
}