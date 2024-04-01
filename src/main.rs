use bevy_reflect::Reflect;
use crate::db::{aio_database::AioDatabase, aio_query::{Next, Operator}};

pub mod db;

#[derive(Reflect, Default, Clone, Debug)]
struct Test {
    name: String,
    test: i32,
    test2: i32,
    test_bool: bool,
    test_bool_false: bool
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let file_db = AioDatabase::create::<Test>("G:\\".into(), "Test".into()).await;

    file_db.insert_value(Test {
        name: "Test".into(),
        test: 0,
        test2: 0,
        test_bool: true,
        test_bool_false: false
    }).await;

    let get_record = file_db
        .query()
        .field("test")
        .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
        .field("name")
        .where_is(Operator::Eq("Test".into()), None)
        .get_many_values::<Test>().await;

    println!("Record result: {:?}", get_record);

    let update_rows = file_db
        .query()
        .field("test")
        .where_is(Operator::Gt((-1).to_string()), Some(Next::Or))
        .update_value(Test {
            name: "Test".into(),
            test: 5,
            test2: 5,
            test_bool: false,
            test_bool_false: true
        }).await;

    println!("Updated rows: {:?}", update_rows);

    let delete_rows = file_db
        .query()
        .field("name")
        .where_is(Operator::Eq("Test".into()), None)
        .delete_value::<Test>().await;

    println!("Deleted rows: {:?}", delete_rows);
}