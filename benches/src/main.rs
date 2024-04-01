use std::fs::{self, File};

use rs_aio_db::db::aio_query::{Next, Operator, QueryBuilder};
use rs_aio_db::db::aio_database::AioDatabase;
use rs_aio_db::Reflect;

#[derive(Default, Clone, Debug, Reflect)]
struct Person {
    id: u32,
    first_name: String,
    last_name: String,
    age: u32,
    height: f32,
    married: bool,
    address: String,
    date_of_birth: u32,
    comments: String
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();

    _ = fs::remove_file("G:\\Test.db");

    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;

    let mut sw = stopwatch::Stopwatch::start_new();

    for i in 0..1000 {
        let person = Person {
            id: i,
            first_name: "Mylo".into(),
            last_name: "Lastnamsky".into(),
            age: 50,
            height: 2.10,
            married: false,
            address: "North Pole, Ice Street 0, NP0001".into(),
            date_of_birth: 1000000,
            comments: "It's very cold up there. Send help!".into()
        };

        file_db.insert_value(person).await;
    }

    println!("Time elapsed for inserting 1000 persons: {}ms", sw.elapsed_ms());

    sw.restart();

    for i in 0..1000 {
        let person = Person {
            id: i,
            first_name: "Mylo 2".into(),
            last_name: "Lastnamsky 2".into(),
            age: 100,
            height: 4.20,
            married: true,
            address: "North Pole, Ice Street 0, NP0001".into(),
            date_of_birth: 2000000,
            comments: "It s very cold up there. Send help!".into()
        };

        file_db
            .query()
            .field("id")
            .where_is(Operator::Eq(i.to_string()), None)
            .update_value(person)
            .await;
    }

    println!("Time elapsed for updating 1000 persons: {}ms", sw.elapsed_ms());

    sw.restart();

    for i in 0..1000 {
        let person = file_db
            .query()
            .field("id")
            .where_is(Operator::Eq(i.to_string()), None)
            .get_single_value::<Person>()
            .await
            .unwrap_or_default();
    }

    println!("Time elapsed for retrieving one by one 1000 persons: {}ms", sw.elapsed_ms());

    sw.restart();

    let persons = file_db
        .query()
        .field("first_name")
        .where_is(Operator::Eq("Mylo 2".into()), None)
        .get_many_values::<Person>()
        .await
        .unwrap();

    println!("Time elapsed for retrieving at once 1000 persons: {}ms", sw.elapsed_ms());
}