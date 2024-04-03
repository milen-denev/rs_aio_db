use rs_aio_db::db::aio_query::{Next, Operator};
use rs_aio_db::db::aio_database::AioDatabase;
mod model;
use actix_web::{get, App, Responder, HttpServer};

//DON'T PLACE WITHIN THE SAME DIRECTORY MODELS AND ACTIX ENDPOINTS
use crate::model::Person;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;

    //In-Memory database
    //let in_memory_db = AioDatabase::create_in_memory::<Person>("Test".into()).await;

    file_db.insert_value(Person {
        name: "Mylo".into(),
        age: 0,
        height: 0,
        married: true
    }).await;

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
            married: false
        }).await;

    println!("Updated rows: {:?}", update_rows);

    let partial_update_rows = file_db
        .query()
        .field("age")
        .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
        .partial_update::<Person>("height".into(), "50".into()).await;

    println!("Updated rows: {:?}", partial_update_rows);

    let delete_rows = file_db
        .query()
        .field("name")
        .where_is(Operator::Eq("Mylo".into()), None)
        .delete_value::<Person>().await;

    println!("Deleted rows: {:?}", delete_rows);

    _ = HttpServer::new(|| {
        App::new().service(index)
    })
    .bind(("127.0.0.1", 80))
    .unwrap()
    .run()
    .await;
}

#[get("/")]
async fn index() -> impl Responder {
   format!("Db Called out!")
}