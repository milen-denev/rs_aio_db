use std::sync::Arc;

use rs_aio_db::db::aio_query::{Next, Operator};
use rs_aio_db::db::aio_database::AioDatabase;
mod model;
use actix_web::{get, web, App, HttpServer, Responder};

//DON'T PLACE WITHIN THE SAME DIRECTORY MODELS AND ACTIX ENDPOINTS
use crate::model::Person;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();
    
    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into(), 15).await;

    //In-Memory database
    let in_memory_db = AioDatabase::create_in_memory::<Person>("Test".into(), 15).await;

    //Remote database for Torso (DB). This project is not affiliated but since it works it's added as an option.
    let remote_db = AioDatabase::create_remote_dont_use_only_for_testing::<Person>(
        "libsql://<SOME_SUBDOMAIN>.turso.io".into(),
        "<SOME_TOKEN>".into(),
        "<SOME_TABLE_NAME>".into()).await;
    
    let mut hash_map = HashMap::new();
    hash_map.insert("Key1".into(), "Value1".into());
    
    _ = file_db.insert_value(&Person {
        name: "Mylo".into(),
        age: 0,
        height: 0,
        married: true,
        some_blob: AioDatabase::get_bytes(AnotherStruct {
            data_1: 5,
            data_2: 10.4,
            data_3:  hash_map.clone()
        })
    }).await.unwrap();

    let get_record = file_db
        .query()
        .field("age")
        .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
        .field("name")
        .where_is(Operator::Eq("Mylo".into()), None)
        .get_many_values::<Person>().await;

    println!("Record result: {:?}", get_record);

    let update_rows = file_db
        .query()
        .field("age")
        .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
        .update_value(Person {
            name: "Mylo".into(),
            age: 0,
            height: 5,
            married: false,
            some_blob: AioDatabase::get_bytes(AnotherStruct {
                data_1: 5,
                data_2: 10.4,
                data_3: hash_map.clone()
            })
        }).await;

    println!("Updated rows: {:?}", update_rows);

    let partial_update_rows = file_db
        .query()
        .field("age")
        .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
        .partial_update::<Person>("height".into(), "50".into()).await;

    println!("Updated rows: {:?}", partial_update_rows);

    _ = file_db.insert_value(&Person {
        name: "Mylo 300".into(),
        age: 0,
        height: 0,
        married: true,
        some_blob: AioDatabase::get_bytes(AnotherStruct {
            data_1: 5,
            data_2: 10.4,
            data_3:  hash_map.clone()
        })
    }).await.unwrap();

    let delete_rows = file_db
        .query()
        .field("name")
        .where_is(Operator::Eq("Mylo 300".into()), None)
        .delete_value::<Person>().await;

    println!("Deleted rows: {:?}", delete_rows);

    let any = file_db
        .query()
        .field("name")
        .where_is(Operator::Ne("Mylo 300".into()), None)
        .any::<Person>()
        .await;

    println!("Any: {:?}", any);

    let count = file_db
        .query()
        .field("name")
        .where_is(Operator::Ne("Mylo 300".into()), None)
        .count::<Person>()
        .await;

    println!("Count: {:?}", count);

    let all = file_db
        .query()
        .field("name")
        .where_is(Operator::Contains("Mylo 900".into()), None)
        .all::<Person>()
        .await;

    println!("All: {:?}", all);

    let arc = Arc::new(file_db);

    _ = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(arc.clone()))
            .service(index)
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .run()
    .await;
}

#[get("/")]
async fn index(pool: web::Data<Arc<AioDatabase>>) -> impl Responder {
    let record = pool
        .query()
        .field("name")
        .where_is(Operator::Ne("Not Mylo".into()), None)
        .get_single_value::<Person>()
        .await
        .unwrap();

   format!("Db Called out! First retrieved record: {}", record.name)
}