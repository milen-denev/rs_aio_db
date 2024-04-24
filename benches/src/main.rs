use std::{collections::HashMap, fs};

use rs_aio_db::db::aio_query::Operator;
use rs_aio_db::db::aio_database::AioDatabase;

use crate::model::{AnotherStruct, Person};
mod model;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "error");
    env_logger::init();

    _ = fs::remove_file("G:\\Test.db");

    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into(), 15).await;

    let mut sw = stopwatch::Stopwatch::start_new();

    const TOTAL_ITERATIONS: u32 = 10;

    let mut hash_map = HashMap::new();
    hash_map.insert("Key1".into(), "Value1".into());


    for i in 0..TOTAL_ITERATIONS {
        let person = Person {
            id: i,
            first_name: "Mylo".into(),
            last_name: "Lastnamsky".into(),
            age: 50,
            height: 2.10,
            married: false,
            address: "North Pole, Ice Street 0, NP0001".into(),
            date_of_birth: 1000000,
            comments: "It's very cold up there. Send help!".into(),
            some_blob: AioDatabase::get_bytes(AnotherStruct {
                data_1: 5,
                data_2: 10.4,
                data_3:  hash_map.clone()
            })
        };

        _ = file_db.insert_value(&person).await.unwrap();
    }

    println!("Time elapsed for inserting {} persons: {}ms", TOTAL_ITERATIONS, sw.elapsed_ms());

    sw.restart();

    for i in 0..TOTAL_ITERATIONS {
        let person = Person {
            id: i,
            first_name: "Mylo 2".into(),
            last_name: "Lastnamsky 2".into(),
            age: 100,
            height: 4.20,
            married: true,
            address: "North Pole, Ice Street 0, NP0001".into(),
            date_of_birth: 2000000,
            comments: "It s very cold up there. Send help!".into(),
            some_blob: AioDatabase::get_bytes(AnotherStruct {
                data_1: 5,
                data_2: 10.4,
                data_3:  hash_map.clone()
            })
        };

        _ = file_db
            .query()
            .field("id")
            .where_is(Operator::Eq(i.to_string()), None)
            .update_value(person)
            .await
            .unwrap();
    }

    println!("Time elapsed for updating {} persons: {}ms", TOTAL_ITERATIONS, sw.elapsed_ms());

    sw.restart();

    for i in 0..TOTAL_ITERATIONS {
        let person = file_db
            .query()
            .field("id")
            .where_is(Operator::Eq(i.to_string()), None)
            .get_single_value::<Person>()
            .await
            .unwrap_or_default();

        let test = person.address;
        drop(test);
    }

    println!("Time elapsed for retrieving one by one {} persons: {}ms", TOTAL_ITERATIONS, sw.elapsed_ms());

    sw.restart();

    let persons = file_db
        .query()
        .field("first_name")
        .where_is(Operator::Eq("Mylo 2".into()), None)
        .get_many_values::<Person>()
        .await
        .unwrap();

    let length = persons.len();
    
    #[allow(dropping_copy_types)]
    drop(length);

    println!("{:?}", persons);

    println!("Time elapsed for retrieving at once {} persons: {}ms", TOTAL_ITERATIONS, sw.elapsed_ms());
}