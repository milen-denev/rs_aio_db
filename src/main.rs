use bevy_reflect::Reflect;
use db::aio_query::Query;
use crate::db::aio_database::AioDatabase;

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

    file_db.insert_value(Test {
        name: "Test".into(),
        test: 15,
        test2: 16
    }).await;

    let test = file_db.get_single_value::<Test>(Query { final_query_str: "".into() }).await;

    println!("{:?}", test);

    
}