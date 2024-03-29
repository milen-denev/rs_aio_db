use bevy_reflect::Reflect;
use db::libsql::AioDatabase;
use bevy_reflect::Struct;

pub mod db;

#[derive(Reflect, Default)]
struct Test {
    name: String,
    test: i32,
    test2:i32
}

#[tokio::main]
async fn main() {
    AioDatabase::create::<Test>("G:\\".into(), "test.db".into()).await;
}