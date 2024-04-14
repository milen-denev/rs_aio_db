use std::{collections::HashMap, fs};

use rs_aio_db::{db::{aio_database::AioDatabase, aio_query::Operator}, Reflect};
use serde::{Deserialize, Serialize};
use tokio::runtime;

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
    comments: String,
    some_blob: Vec<u8>
}

#[derive(Serialize, Deserialize)]
struct AnotherStruct {
    pub data_1: i32,
    pub data_2: f64,
    pub data_3: HashMap<String, String>
}

#[test]
fn create_db() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\create_db.db");

          _ = AioDatabase::create::<Person>("G:\\".into(), "create_db".into(), 15).await;
          let result = fs::File::open("G:\\create_db.db");

          assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn insert_value() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\insert_value.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "insert_value".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: false,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;

          let result = fs::File::open("G:\\insert_value.db");

          assert_eq!(result.is_ok(), true);
    });
}

#[test]
fn update_value() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\update_value.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "update_value".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());


          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: false,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;

          let rows = file_db
               .query()
               .field("id")
               .where_is(Operator::Eq((0).to_string()), None)
               .update_value(person)
               .await;

          assert_eq!(rows, 1);
    });
}

#[test]
fn retrieve_single_value() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\retrieve_single_value.db");
          let file_db = AioDatabase::create::<Person>("G:\\".into(), "retrieve_single_value".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;

          let retrieved_person = file_db
               .query()
               .field("id")
               .where_is(Operator::Eq((0).to_string()), None)
               .get_single_value::<Person>()
               .await
               .unwrap_or_default();

          assert_eq!(retrieved_person.id, 0);
          assert_eq!(retrieved_person.first_name, "Mylo");
          assert_eq!(retrieved_person.comments, "It^s very cold up there. Send help!");
          assert_eq!(retrieved_person.married, true);
          assert_eq!(retrieved_person.height, 2.10);
          assert_eq!(retrieved_person.age, 50);
    });
}

#[test]
fn retrieve_all_values() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\retrieve_all_values.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "retrieve_all_values".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;

          let retrieved_persons: Vec<Person> = file_db
               .query()
               .field("married")
               .where_is(Operator::Eq((true).to_string()), None)
               .get_many_values()
               .await
               .unwrap();

          assert_eq!(retrieved_persons[0].id, 0);
          assert_eq!(retrieved_persons[0].first_name, "Mylo");
          assert_eq!(retrieved_persons[0].comments, "It^s very cold up there. Send help!");
          assert_eq!(retrieved_persons[0].married, true);
          assert_eq!(retrieved_persons[0].height, 2.10);
          assert_eq!(retrieved_persons[0].age, 50);

          assert_eq!(retrieved_persons[1].id, 1);
          assert_eq!(retrieved_persons[1].first_name, "Mylo 2");

          assert_eq!(retrieved_persons.len(), 4);
    });
}

#[test]
fn delete_all_values() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\delete_all_values.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "delete_all_values".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very cold up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;

          let deleted_persons: u64 = file_db
               .query()
               .field("married")
               .where_is(Operator::Eq((true).to_string()), None)
               .delete_value::<Person>()
               .await;

          assert_eq!(deleted_persons, 4);  
    });
}

#[test]
fn contains_values() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\contains_values.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "contains_values".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very CoLd up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let retrieved_person = file_db
               .query()
               .field("comments")
               .where_is(Operator::Contains(("CoLd").to_string()), None)
               .get_single_value::<Person>()
               .await
               .unwrap();

          assert_eq!(retrieved_person.id, 655);
    });
}

#[test]
fn starts_with_values() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\starts_with_values.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "starts_with_values".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there. It^^s Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^^s very CoLd up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let retrieved_person = file_db
               .query()
               .field("comments")
               .where_is(Operator::StartsWith(("It^^s").to_string()), None)
               .get_single_value::<Person>()
               .await
               .unwrap();

          assert_eq!(retrieved_person.id, 655);
    });
}

#[test]
fn ends_with_values() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\ends_with_values.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "ends_with_values".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there HelP!!. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^^s very CoLd up there. Send HelP!!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let retrieved_person = file_db
               .query()
               .field("comments")
               .where_is(Operator::EndsWith("HelP!!".to_string()), None)
               .get_single_value::<Person>()
               .await
               .unwrap();

          assert_eq!(retrieved_person.id, 655);
    });
}

#[test]
fn any() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\any.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "any".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there. It^^s Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^^s very CoLd up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let any = file_db
               .query()
               .field("comments")
               .where_is(Operator::Contains(("It^^s").to_string()), None)
               .any::<Person>()
               .await;

          assert_eq!(any, true);
    });
}

#[test]
fn all() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\all0.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "all0".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there. It^^s Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^^s very CoLd up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let all = file_db
               .query()
               .field("address")
               .where_is(Operator::Contains(("North Pole").to_string()), None)
               .all::<Person>()
               .await;

          assert_eq!(all, true);
    });
}

#[test]
fn count() {
    let rt = runtime::Builder::new_current_thread().build().unwrap();
    rt.block_on(async { 
          _ = fs::remove_file("G:\\count.db");

          let file_db = AioDatabase::create::<Person>("G:\\".into(), "count".into(), 15).await;

          let mut hash_map = HashMap::new();
          hash_map.insert("Key".into(), "Value1".into());

          let person = Person {
               id: 0,
               first_name: "Mylo".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very hot up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person3 = Person {
               id: 1,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^s very warm up there. It^^s Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          let person2 = Person {
               id: 655,
               first_name: "Mylo 2".into(),
               last_name: "Lastnamsky".into(),
               age: 50,
               height: 2.10,
               married: true,
               address: "North Pole, Ice Street 0, NP0001".into(),
               date_of_birth: 1000000,
               comments: "It^^s very CoLd up there. Send help!".into(),
               some_blob: AioDatabase::get_bytes(AnotherStruct {
                   data_1: 5,
                   data_2: 10.4,
                   data_3:  hash_map.clone()
               })
          };

          file_db.insert_value(&person).await;
          file_db.insert_value(&person2).await;
          file_db.insert_value(&person3).await;

          let count = file_db
               .query()
               .field("address")
               .where_is(Operator::Contains(("North Pole").to_string()), None)
               .count::<Person>()
               .await;

          assert_eq!(count, 3);
    });
}