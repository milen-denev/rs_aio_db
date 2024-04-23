# Aio Database
## All in one database with dead simple API

## Features

- Auto migration: If additional or fewer fields are introduced to a structure, it immediately updates the database schema.
- Local or In-Memory Capability: All functionality operates within local storage or in-memory systems.
- Fully implemented CRUD functionality
- Highly Performant: Offers very good performance, by doing some preliminary tests it seems that the overhead from both main libraries that I use (libsql and bevy_reflect) plus the overhead from my library is small enough to be unnoticeable, reading 1000 rows one by one took 28ms. 
- Async Support with Tokio
- Highly Concurrent due to the internal connection pooling
- ORM-like API that is dead simple to use
- Use anywhere

## Production Readiness 

This is used in production in affiliated company for specific use-case. Use this in production at your own risk. Currently I consider this to in beta stage. There are known bugs, check the pinned Issue's tab. Although there are no observable major issues until v0.6 I don't recommend it for use in critical systems.

## Planned Features

- Use of Moka cache for bypassing the local storage and increase performance.
- Additional Query options.
- Additional options for AioDatabase instances apart from the in-memory and local storage drive.

## Examples

### cargo.toml
```TOML
[dependencies]
rs_aio_db = "0.5.10"
env_logger = "0.11.3"
tokio = "1.37.0"
bevy_reflect = "0.13.1"
serde = "1.0.198"
```

### main.rs
```rust
use rs_aio_db::db::aio_query::{Next, Operator, QueryBuilder};
use rs_aio_db::db::aio_database::AioDatabase;
use rs_aio_db::Reflect;

#[derive(Default, Clone, Debug, Reflect)]
pub struct Person {
     pub name: String,
     pub age: i32,
     pub height: i32,
     pub married: bool,
     pub some_blob: Vec<u8>
}

#[derive(Serialize, Deserialize)]
struct AnotherStruct {
    pub data_1: i32,
    pub data_2: f64,
    pub data_3: HashMap<String, String>
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    //Locally persisted database
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into(), 15).await;

    //In-Memory database
    let in_memory_db = AioDatabase::create_in_memory::<Person>("Test".into(), 15).await;

    let mut hash_map = HashMap::new();
    hash_map.insert("Key1".into(), "Value1".into());

    //Use AioDatabase::get_struct to get back your struct data type

    file_db.insert_value(&Person {
        name: "Mylo".into(),
        age: 0,
        height: 0,
        married: true,
        some_blob: AioDatabase::get_bytes(AnotherStruct {
            data_1: 5,
            data_2: 10.4,
            data_3:  hash_map.clone()
        })
    }).await;

    let get_single_record = file_db
        .query()
        .field("age")
        .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
        .field("name")
        .where_is(Operator::Eq("Mylo".into()), None)
        .get_single_value::<Person>()
        .await
        .unwrap_or_default();

    println!("Record result: {:?}", get_single_record);

    let get_records = file_db
        .query()
        .field("age")
        .where_is(Operator::Gt(5.to_string()), Some(Next::Or))
        .field("name")
        .where_is(Operator::Eq("Mylo".into()), None)
        .get_many_values::<Person>().await;

    println!("Record results: {:?}", get_records);

    let update_rows = file_db
        .query()
        .field("age")
        .where_is(Operator::Eq((0).to_string()), Some(Next::Or))
        .update_value(Person {
            name: "Mylo".into(),
            age: 5,
            height: 5,
            married: false,
            some_blob: AioDatabase::get_bytes(AnotherStruct {
                data_1: 5,
                data_2: 10.4,
                data_3:  hash_map.clone()
            })
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

    let contains = file_db
        .query()
        .field("name")
        .where_is(Operator::Contains("Mylo".into()), None)
        .get_single_value::<Person>()
        .await
        .unwrap_or_default();

    println!("Contains: {:?}", contains);

    let starts_with = file_db
        .query()
        .field("name")
        .where_is(Operator::StartsWith("Mylo".into()), None)
        .get_single_value::<Person>()
        .await
        .unwrap_or_default();

    println!("Starts with: {:?}", starts_with);

    
    let starts_with = file_db
        .query()
        .field("name")
        .where_is(Operator::EndsWith("Mylo".into()), None)
        .get_single_value::<Person>()
        .await
        .unwrap_or_default();

    println!("Ends with: {:?}", starts_with);
}
```

### Benchmarks

#### Figure 1
![image](https://github.com/milen-denev/rs_aio_db/blob/master/benches/images/benchmark_02042023.jpg)

#### Figure 2
![image](https://github.com/milen-denev/rs_aio_db/blob/master/benches/images/high_con_perf_03042024.jpg)

#### Explanation

**First Image:**
All of this 4 benchmarks has been done synchronously. The point of synchronously executing 1000 times each test was to see how much overhead does my library add to libsql and bevy_reflect. As it seems from the 3rd test which executed 1000 times not much (**28ms**). For retrieving 1 row it took on average 0.0028ms or 28us which is fast. Let's not forget the latency of the SSD itself and the Sqlite engine which for sure adds more to the equation. When executed the first and second test scenario my SSD reached latency of 21.1ms and 90% usage for sure is the reason behind the 3+ seconds for 1000 row inserts and row updates. It's under investigation.

**Second Image:**
The image shows the result of a K6 on actix-web + AioDatabase setup. It performed insanely well on 5000 concurrent connections with a pool size of 15. The code behind this can be be found under **/example** folder within the repository.
