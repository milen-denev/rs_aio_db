# Aio Database
## All in one database with dead simple API

## Features

- Auto migration: If additional or fewer fields are introduced to a structure, it immediately updates the required alterations to the database schema.
- Local or In-Memory Capability: All functionality operates within local storage or in-memory systems.
- Create records, retrieve one or many, update them or delete them with a dead simple ORM-like API.
- Performance: Offers very good performance, by doing some preliminary tests it seems that the overhead from both main libraries that I use (libsql and bevy_reflect) plus the overhead from my library is small enough to be unnoticeable, reading 1000 rows one by one took 28ms. 
- Async Support with Tokio

## Examples 

### cargo.toml
```TOML
[dependencies]
rs_aio_db = "0.5.1"
env_logger = "0.11.3"
tokio = "1.37.0"
bevy_reflect = "0.13.1"
```

### main.rs
```rust
use rs_aio_db::db::aio_query::{Next, Operator, QueryBuilder};
use rs_aio_db::db::aio_database::AioDatabase;
use rs_aio_db::Reflect;

#[derive(Default, Clone, Debug, Reflect)]
struct Person {
    name: String,
    age: i32,
    height: i32,
    married: bool,
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;

    //In-Memory database
    let in_memory_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;

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
            age: 5,
            height: 5,
            married: false
        }).await;

    println!("Updated rows: {:?}", update_rows);

    let delete_rows = file_db
        .query()
        .field("name")
        .where_is(Operator::Eq("Mylo".into()), None)
        .delete_value::<Person>().await;

    println!("Deleted rows: {:?}", delete_rows);
}
```
### Benchmarks
![image](https://github.com/milen-denev/rs_aio_db/blob/master/benches/images/benchmark_02042023.jpg)
##### Explanation
All of this 4 benchmarks has been done synchronously. The point of synchronously executing 1000 times each test was to see how much overhead does my library add to libsql and bevy_reflect. As it seems from the 3rd test which executed 1000 times not much (**28ms**). For retrieving 1 row it took on average 0.0028ms or 28us which is fast. Let's not forget the latency of the SSD itself and the Sqlite engine which for sure adds more to the equation. When executed the first and second test scenario my SSD reached latency of 21.1ms and 90% usage for sure is the reason behind the 3+ seconds for 1000 row inserts and row updates. It's under investigation.
