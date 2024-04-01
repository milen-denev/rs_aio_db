# Aio Database
## All in one database with dead simple API

## Features

- Auto migration: If additional or fewer fields are introduced to a structure, it immediately updates the required alterations to the database schema.
- Local or In-Memory Capability: All functionality operates within local storage or in-memory systems.
- Create records, retrieve one or many, update them or delete them with a dead simple ORM-like API.
- Performance: To be determined (relies on bevy_reflect and libsql)
- Async Support with Tokio

## Examples 

```rust
#[derive(Reflect, Default, Clone, Debug)]
struct Person {
    name: String,
    age: i32,
    height: i32,
    married: bool,
}

#[tokio::main]
async fn main() {
    let file_db = AioDatabase::create::<Person>("G:\\".into(), "Test".into()).await;

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
