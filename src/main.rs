use bevy_reflect::Reflect;
use crate::db::{aio_database::AioDatabase, aio_query::Operator};

pub mod db;

#[derive(Reflect, Default, Clone, Debug)]
struct Test {
    name: String,
    test: i32,
    test2: i32
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let file_db = AioDatabase::create::<Test>("G:\\".into(), "Test".into()).await;

    let result = file_db
        .query()
        .field("test")
        .where_is(Operator::Lt(5.to_string()), None)
        .get_single_value::<Test>().await;

    println!("{:?}", result);
}